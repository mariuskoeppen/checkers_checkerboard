// /* Program looks for files "DB6" and "DB6.idx" in the current directory */
// /* If you want to change the file names, look for them in the code.     */

// /* Note: this code works for the 6p-databases, but may require some	*/
// /* modifications for the 8p-databases.					*/

// /* Look at DB_BUFFERS to set the storage requirements you want to use.  */

// /* Code has been modified to reduce storage requirements.  With more    */
// /* storage, you can rewrite to get more efficient code.                 */

// /* Note: code as given only gives the correct result for a position     */
// /* where the side to move does not have a capture move, or if the turn  */
// /* is switched, then a capture move would be present.                   */
// /*                                                                      */
// /* Incorrect result returned because white can capture:                 */
// /*                                                                      */
// /* 8  . - . - . - . -                                                   */
// /* 7  - . - . - . - .                                                   */
// /* 6  . - . - . - . -                                                   */
// /* 5  - . b . b . - .   White to move                                   */
// /* 4  . w . - . - . -                                                   */
// /* 3  w . - . - . - .                                                   */
// /* 2  . - . - . - . -                                                   */
// /* 1  - . - . - . - .                                                   */
// /*                                                                      */
// /* Incorrect result returned because if the turn were switched to       */
// /* black, then a capture would be possible.                             */
// /* 8  . - . - . - . -                                                   */
// /* 7  - . - . - . - .                                                   */
// /* 6  . - . b . - . -                                                   */
// /* 5  - . b . - . - .   White to move                                   */
// /* 4  . w . - . - . -                                                   */
// /* 3  - . w . - . - .                                                   */
// /* 2  . - . - . - . -                                                   */
// /* 1  - . - . - . - .                                                   */

// /*
//  * Table to map between DB board representation and Chinook's
//  * Note: the colours are on opposite sides
//  *
//  *       Database Board:         Chinook's Board:
//  *             WHITE                  BLACK
//  *         28  29  30  31          7  15  23  31
//  *       24  25  26  27          3  11  19  27
//  *         20  21  22  23          6  14  22  30
//  *       16  17  18  19          2  10  18  26
//  *         12  13  14  15          5  13  21  29
//  *        8   9  10  11          1   9  17  25
//  *          4   5   6   7          4  12  20  28
//  *        0   1   2   3          0   8  16  24
//  *            BLACK                    WHITE
//  */
// /* Note that if your board representation is already the left one, you  */
// /* do not need the code to convert it to the right one.                 */

// /* Index file contains all the sub-database - file is in text   */
//         /* each db is in there, and this loop reads in that info.  For  */
//         /* example, here is the head of the file:                       */
//         /* BASE1100.00 =        DB 1bk, 1wk, 0bp, 0wp - mostly draws    */
//         /* S      0       0/0   Start position 0 at block 0, byte 0     */
//         /* E    995       0/189 End at position 995, at block 0 byte 189*/
//         /*                      By defualt, block size is 1024 bytes    */
//         /*                                                              */
//         /*                      This says this database of 995 positions*/
//         /*                      has been compressed to 189 bytes.       */
//         /*                                                              */
//         /* BASE1001.00 +        DB 1bk, 0wk, 0bp, 1wp on rank 0. Wins   */
//         /* S      0       0/189                                         */
//         /* E    125       0/214                                         */
//         /*                      125 positions in 214-189=25 bytes       */
//         /*                      Most  positions in DB are black wins    */
//         /*                                                              */
//         /* BASE1001.01 =        DB 1bk, 0wk, 0bp, 1wp on rank 1.  Draws */
//         /* S      0       0/214                                         */
//         /* etc.                 Interesting.  With the checker on the   */
//         /*                      1st rank most pos are draws, on the 0th,*/
//         /*                      wins.  Must be because of the move.     */
//         /*                                                              */
//         /* BASE0011.51 ==       DB 0 kings, 1 bp on 5th, 1 wp on 1st    */
//         /*                      == means entire db is drawn.  ++ means  */
//         /*                      all db is win; -- all is loss.          */
//         /*                                                              */
//         /* Here is a bigger db:                                         */
//         /* BASE1311.26 -                                                */
//         /* S      0    2024/783 Position 0 at block 2024, byte 783      */
//         /*                      i.e. starts at addr=2024*1024+783       */
//         /* . 327765    2025     Next block (2025), 1st pos is 327,765   */
//         /* .1625795    2026     Next block (2026), 1st pos is 1,625,795 */
//         /* E1753920    2026/143 Ending addr=2026*1024+143               */
//         /*                      Num positions in db = 1,753,920         */
//         /*                      Bytes used: endaddr-startaddr=1408      */
//         /*                      Nice compression ratio!!                */


// /*
//  * A database slice is determined by the leading rank of the black and white
//  * checkers.  A position index within the slice is computed by the position
//  * of the black checkers, then adding the white checkers, followed by the
//  * black kings, and finally the white kings.
//  *
//  * The following "|" notation refers to "choose";
//  *  i.e. | n | = n!/((n-k)!k!).
//  *       | k |
//  *
//  * The number of positions with nbp black checkers with leading rank rbp
//  * (where 0 <= rbp <= 7) is MaxBP = | 4 x (rbp+1) | - | 4 x rbp |.
//  *                                  |     nbp     |   |  nbp    |
//  *
//  * Next the white checkers are added.  This is discussed in more detail below.
//  *
//  * The number of positions created by adding nbk black kings is
//  *      MaxBK = | 32 - nbp - nwp |
//  *              |      nbk       |
//  *
//  * and the number of positions created by adding nwk white kings is
//  *      MaxWK = | 32 - nbp - nwp - nbk |.
//  *              |         nwk          |
//  *
//  *
//  * Computing the number of positions with nbp black checkers having leading
//  * rank rbp and nwp white checkers having leading rank rwp is difficult because
//  * some of the black checker configurations may have one or more checkers
//  * within the first rwp checker squares.  Thus the simple formula applied to
//  * computing MaxBP cannot be applied to MaxWP because different black checker
//  * configurations may have a different number of white checker configurations.
//  * MaxWP is computed by creating a set of subindexes for every black checker
//  * configuration.  The number of white checker positions for each black checker
//  * configuration is computed and stored, as well as a cumulative total.  The
//  * amount of storage required can be quite large for a large number of black
//  * checkers.
//  *
//  * See Appendix B in the Technical Report TR93-13 for more details.
//  */

