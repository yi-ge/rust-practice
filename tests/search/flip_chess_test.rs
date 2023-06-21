use rust_practice::search::flip_chess::Solution;

#[test]
fn flip_chess() {
    // 示例 1：
    // 输入：chessboard = ["....X.","....X.","XOOO..","......","......"]
    // 输出：3
    // 解释： 可以选择下在 [2,4] 处，能够翻转白方三枚棋子。
    let chessboard = vec![
        String::from("....X."),
        String::from("....X."),
        String::from("XOOO.."),
        String::from("......"),
        String::from("......"),
    ];
    assert_eq!(Solution::flip_chess(chessboard), 3);

    // 示例 2：
    // 输入：chessboard = [".X.",".O.","XO."]
    // 输出：2
    // 解释： 可以选择下在 [2,2] 处，能够翻转白方两枚棋子。
    let chessboard = vec![
        String::from(".X."),
        String::from(".O."),
        String::from("XO."),
    ];
    assert_eq!(Solution::flip_chess(chessboard), 2);

    // 示例 3：
    // 输入：chessboard = [".......",".......",".......","X......",".O.....","..O....","....OOX"]
    // 输出：4
    // 解释： 可以选择下在 [6,3] 处，能够翻转白方四枚棋子。
    let chessboard = vec![
        String::from("......."),
        String::from("......."),
        String::from("......."),
        String::from("X......"),
        String::from(".O....."),
        String::from("..O...."),
        String::from("....OOX"),
    ];
    assert_eq!(Solution::flip_chess(chessboard), 4);
}
