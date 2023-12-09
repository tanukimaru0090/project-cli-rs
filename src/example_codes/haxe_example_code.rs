pub const HAXE_EXAMPLE_CODE_1: &str = r#"
class Main{
    static function main(){
        trace("hello world!\n");
    }
}
"#;
pub const HAXE_EXAMPLE_CODE_2: &str = r#"
class Main{
    static function main(){
        var fruit:Array<String> = ["Apple","Orange","Banana","Grape"];
        for(value in fruit){
            trace("Fruit: ");
            trace(value+"\n");
        }
    }
}
"#;
pub const HAXE_BUILD_DEFAULT_CODE: &str = r#"
-cp src 
-main Main
"#;
pub const HAXE_EXAMPLE_CODE_3: &str = r#""#;
pub const HAXE_EXAMPLE_CODE_NONE: &str = "";
