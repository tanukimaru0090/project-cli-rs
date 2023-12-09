pub const CPP_EXAMPLE_CODE_1: &str = r#"
#include <iostream> 
int main(){
    std::cout << "hello world!" << std::endl;
}
"#;
pub const CPP_EXAMPLE_CODE_2: &str = r#"
/****Project-CLI Sample Code****/
/****https://github.com/tanukimaru0090/project-cli****/
#include <iostream>
#include <string>
#include <vector>

int main(){
    std::vector<std::string>fruit;
    fruit.push_back("Apple");
    fruit.push_back("Orange");
    fruit.push_back("Banana");
    fruit.push_back("Grape");
    for(auto&& value:fruit){
        std::cout << "Fruit: " << value << std::endl;
    }
}
"#;
pub const CPP_EXAMPLE_CODE_3: &str = r#"
#include <iostream> 
int main(){
    std::cout << "hello world!" << std::endl;
}
"#;

pub const CMAKE_DEFAULT_CODE: &str = r#"cmake_minimum_required(VERSION 3.0)
"#;
