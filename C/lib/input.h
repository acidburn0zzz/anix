/*Copyright (C) 2018 Nicolas Fouquet

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see https://www.gnu.org/licenses.
*/

void scanCodeToChar(int i){
    //AZERTY
    switch(i){
        case 0x02:
            print("1");
            break;
        case 0x03:
            print("2");
            break;
        case 0x04:
            print("3");
            break;
        case 0x05:
            print("4");
            break;
        case 0x06:
            print("5");
            break;
        case 0x07:
            print("6");
            break;
        case 0x08:
            print("7");
            break;
        case 0x09:
            print("8");
            break;
        case 0x0a:
            print("9");
            break;
        case 0x0b:
            print("0");
            break;
            
        case 0x10:
            print("a");
            break;
        case 0x11:
            print("z");
            break;
        case 0x12:
            print("e");
            break;
        case 0x13:
            print("r");
            break;
        case 0x14:
            print("t");
            break;
        case 0x15:
            print("y");
            break;
        case 0x16:
            print("u");
            break;
        case 0x17:
            print("i");
            break;
        case 0x18:
            print("o");
            break;
        case 0x19:
            print("p");
            break;
        case 0x1e:
            print("q");
            break;
        case 0x1f:
            print("s");
            break;
        case 0x20:
            print("d");
            break;
        case 0x21:
            print("f");
            break;
        case 0x22:
            print("g");
            break;
        case 0x23:
            print("h");
            break;
        case 0x24:
            print("j");
            break;
        case 0x25:
            print("k");
            break;
        case 0x26:
            print("l");
            break;
        case 0x2c:
            print("w");
            break;
        case 0x2d:
            print("x");
            break;
        case 0x2e:
            print("c");
            break;
        case 0x2f:
            print("v");
            break;
        case 0x30:
            print("b");
            break;
        case 0x31:
            print("n");
            break;
            
        case 0x32:
            print(",");
            break;
            
        case 0x27:
            print("m");
            break;
        case 0x28:
            print("'");
            break;
        case 0x2b:
            print("\\");
            break;
        case 0x33:
            print(";");
            break;
        case 0x34:
            print(":");
            break;
        case 0x1a:
            print("[");
            break;
        case 0x1b:
            print("]");
            break;
        case 0x0d:
            print("=");
            break;
        case 0x0c:
            print("-");
            break;
            
    }
}
