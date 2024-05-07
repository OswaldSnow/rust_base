fn main(){
   /*
   枚举类型
    */

    

    // Option 枚举

}

enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

enum PokerCard {
    Clubs(u8),
    Spades(u8),
    Diamonds(char),
    Hearts(char),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}