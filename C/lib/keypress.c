/*char key[100] = {};
void * function[100];

int numberKeys = 0;
int numberFunctions = 0;

int array_length(int a[]){
    return sizeof(a)/sizeof(int);
}
*/
char keys[0] = {};
void * functions[0] = {};

int numberKeys = 1;
int numberFunctions = 1;

void keypress(char k, void * callback()){
    keys[numberKeys] = "a";
    print("Result : ");
    terminal_putchar(keys[numberKeys]);
    /*char key[0] = "a";
    if(key[0] == "a"){
        print("Hello !");
        //keys.function[0]();
    }
    
    function[numberFunctions] = *callback;
    
    numberKeys += 1;
    numberFunctions += 1;*/
} 

void pressTouch(int touch){
    /*int i;
    switch(touch){
        case 0x10:
            for(i = 0; i < (array_length(key)); i += 2){
                if(key[0] == "a"){
                    print("Hello !");
                    //keys.function[0]();
                }
            }
            break;
    }*/
}
