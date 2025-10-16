pub fn s1(a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a6: u64) -> (u64, u64, u64, u64) {
    let (
        mut x55005500,
        mut x5A0F5A0F,
        mut x3333FFFF,
        mut x66666666,
        mut x22226666,
        mut x2D2D6969,
        mut x25202160,
        mut x00FFFF00,
        mut x33CCCC33,
        mut x4803120C,
        mut x2222FFFF,
        mut x6A21EDF3,
        mut x4A01CC93,
        mut x5555FFFF,
        mut x7F75FFFF,
        mut x00D20096,
        mut x7FA7FF69,
        mut x0A0A0000,
        mut x0AD80096,
        mut x00999900,
        mut x0AD99996,
        mut x22332233,
        mut x257AA5F0,
        mut x054885C0,
        mut xFAB77A3F,
        mut x2221EDF3,
        mut xD89697CC,
        mut x05B77AC0,
        mut x05F77AD6,
        mut x36C48529,
        mut x6391D07C,
        mut xBB0747B0,
        mut x4C460000,
        mut x4EDF9996,
        mut x2D4E49EA,
        mut xBBFFFFB0,
        mut x96B1B65A,
        mut x5AFF5AFF,
        mut x52B11215,
        mut x4201C010,
        mut x10B0D205,
        mut x00,
        mut x01,
        mut x10,
        mut x11,
        mut x20,
        mut x21,
        mut x30,
        mut x31,
    ) = (
        0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
        0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
        0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
        0u64, 0u64, 0u64, 0u64,
    );

    let mut out1 = 0u64;
    let mut out2 = 0u64;
    let mut out3 = 0u64;
    let mut out4 = 0u64;

    x55005500 = a1 & !a5;
    x5A0F5A0F = a4 ^ x55005500;
    x3333FFFF = a3 | a6;
    x66666666 = a1 ^ a3;
    x22226666 = x3333FFFF & x66666666;
    x2D2D6969 = a4 ^ x22226666;
    x25202160 = x2D2D6969 & !x5A0F5A0F;

    x00FFFF00 = a5 ^ a6;
    x33CCCC33 = a3 ^ x00FFFF00;
    x4803120C = x5A0F5A0F & !x33CCCC33;
    x2222FFFF = a6 | x22226666;
    x6A21EDF3 = x4803120C ^ x2222FFFF;
    x4A01CC93 = x6A21EDF3 & !x25202160;

    x5555FFFF = a1 | a6;
    x7F75FFFF = x6A21EDF3 | x5555FFFF;
    x00D20096 = a5 & !x2D2D6969;
    x7FA7FF69 = x7F75FFFF ^ x00D20096;

    x0A0A0000 = a4 & !x5555FFFF;
    x0AD80096 = x00D20096 ^ x0A0A0000;
    x00999900 = x00FFFF00 & !x66666666;
    x0AD99996 = x0AD80096 | x00999900;

    x22332233 = a3 & !x55005500;
    x257AA5F0 = x5A0F5A0F ^ x7F75FFFF;
    x054885C0 = x257AA5F0 & !x22332233;
    xFAB77A3F = !x054885C0;
    x2221EDF3 = x3333FFFF & x6A21EDF3;
    xD89697CC = xFAB77A3F ^ x2221EDF3;
    x20 = x7FA7FF69 & !a2;
    x21 = x20 ^ xD89697CC;
    out3 ^= x21;

    x05B77AC0 = x00FFFF00 ^ x054885C0;
    x05F77AD6 = x00D20096 | x05B77AC0;
    x36C48529 = x3333FFFF ^ x05F77AD6;
    x6391D07C = a1 ^ x36C48529;
    xBB0747B0 = xD89697CC ^ x6391D07C;
    x00 = x25202160 | a2;
    x01 = x00 ^ xBB0747B0;
    out1 ^= x01;

    x4C460000 = x3333FFFF ^ x7F75FFFF;
    x4EDF9996 = x0AD99996 | x4C460000;
    x2D4E49EA = x6391D07C ^ x4EDF9996;
    xBBFFFFB0 = x00FFFF00 | xBB0747B0;
    x96B1B65A = x2D4E49EA ^ xBBFFFFB0;
    x10 = x4A01CC93 | a2;
    x11 = x10 ^ x96B1B65A;
    out2 ^= x11;

    x5AFF5AFF = a5 | x5A0F5A0F;
    x52B11215 = x5AFF5AFF & !x2D4E49EA;
    x4201C010 = x4A01CC93 & x6391D07C;
    x10B0D205 = x52B11215 ^ x4201C010;
    x30 = x10B0D205 | a2;
    x31 = x30 ^ x0AD99996;
    out4 ^= x31;

    (out1, out2, out3, out4)
}

pub fn s2(a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a6: u64) -> (u64, u64, u64, u64) {
    let (
        mut x33CC33CC,
        mut x55550000,
        mut x00AA00FF,
        mut x33BB33FF,
        mut x33CC0000,
        mut x11441144,
        mut x11BB11BB,
        mut x003311BB,
        mut x00000F0F,
        mut x336600FF,
        mut x332200FF,
        mut x332200F0,
        mut x0302000F,
        mut xAAAAAAAA,
        mut xA9A8AAA5,
        mut x33CCCC33,
        mut x33CCC030,
        mut x9A646A95,
        mut x00333303,
        mut x118822B8,
        mut xA8208805,
        mut x3CC3C33C,
        mut x94E34B39,
        mut x0331330C,
        mut x3FF3F33C,
        mut xA9DF596A,
        mut xA9DF5F6F,
        mut x962CAC53,
        mut xA9466A6A,
        mut x3DA52153,
        mut x29850143,
        mut x33C0330C,
        mut x1A45324F,
        mut x0A451047,
        mut xBBDFDD7B,
        mut xB19ACD3C,
        mut x00,
        mut x01,
        mut x10,
        mut x11,
        mut x20,
        mut x21,
        mut x30,
        mut x31,
    ) = (
        0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
        0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
        0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    );

    let mut out1 = 0u64;
    let mut out2 = 0u64;
    let mut out3 = 0u64;
    let mut out4 = 0u64;

    x33CC33CC = a2 ^ a5;

    x55550000 = a1 & !a6;
    x00AA00FF = a5 & !x55550000;
    x33BB33FF = a2 | x00AA00FF;

    x33CC0000 = x33CC33CC & !a6;
    x11441144 = a1 & x33CC33CC;
    x11BB11BB = a5 ^ x11441144;
    x003311BB = x11BB11BB & !x33CC0000;

    x00000F0F = a3 & a6;
    x336600FF = x00AA00FF ^ x33CC0000;
    x332200FF = x33BB33FF & x336600FF;
    x332200F0 = x332200FF & !x00000F0F;

    x0302000F = a3 & x332200FF;
    xAAAAAAAA = !a1;
    xA9A8AAA5 = x0302000F ^ xAAAAAAAA;
    x33CCCC33 = a6 ^ x33CC33CC;
    x33CCC030 = x33CCCC33 & !x00000F0F;
    x9A646A95 = xA9A8AAA5 ^ x33CCC030;
    x10 = a4 & !x332200F0;
    x11 = x10 ^ x9A646A95;
    out2 ^= x11;

    x00333303 = a2 & !x33CCC030;
    x118822B8 = x11BB11BB ^ x00333303;
    xA8208805 = xA9A8AAA5 & !x118822B8;
    x3CC3C33C = a3 ^ x33CCCC33;
    x94E34B39 = xA8208805 ^ x3CC3C33C;
    x00 = x33BB33FF & !a4;
    x01 = x00 ^ x94E34B39;
    out1 ^= x01;

    x0331330C = x0302000F ^ x00333303;
    x3FF3F33C = x3CC3C33C | x0331330C;
    xA9DF596A = x33BB33FF ^ x9A646A95;
    xA9DF5F6F = x00000F0F | xA9DF596A;
    x962CAC53 = x3FF3F33C ^ xA9DF5F6F;

    xA9466A6A = x332200FF ^ x9A646A95;
    x3DA52153 = x94E34B39 ^ xA9466A6A;
    x29850143 = xA9DF5F6F & x3DA52153;
    x33C0330C = x33CC33CC & x3FF3F33C;
    x1A45324F = x29850143 ^ x33C0330C;
    x20 = x1A45324F | a4;
    x21 = x20 ^ x962CAC53;
    out3 ^= x21;

    x0A451047 = x1A45324F & !x118822B8;
    xBBDFDD7B = x33CCCC33 | xA9DF596A;
    xB19ACD3C = x0A451047 ^ xBBDFDD7B;
    x30 = x003311BB | a4;
    x31 = x30 ^ xB19ACD3C;
    out4 ^= x31;

    (out1, out2, out3, out4)
}

pub fn s3(a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a6: u64) -> (u64, u64, u64, u64) {
    let (mut x44444444, mut x0F0FF0F0, mut x4F4FF4F4, mut x00FFFF00, mut x00AAAA00, mut x4FE55EF4);
    let (mut x3C3CC3C3, mut x3C3C0000, mut x7373F4F4, mut x0C840A00);
    let (mut x00005EF4, mut x00FF5EFF, mut x00555455, mut x3C699796);
    let (mut x000FF000, mut x55AA55AA, mut x26D9A15E, mut x2FDFAF5F, mut x2FD00F5F);
    let (mut x55AAFFAA, mut x28410014, mut x000000FF, mut x000000CC, mut x284100D8);
    let (mut x204100D0, mut x3C3CC3FF, mut x1C3CC32F, mut x4969967A);
    let (mut x4CC44CC4, mut x40C040C0, mut xC3C33C3C, mut x9669C396, mut xD6A98356);
    let (mut xD6E9C3D6, mut x4CEEEEC4, mut x9A072D12, mut x001A000B, mut x9A1F2D1B);
    let (mut x00, mut x01, mut x10, mut x11, mut x20, mut x21, mut x30, mut x31);

    let mut out1 = 0u64;
    let mut out2 = 0u64;
    let mut out3 = 0u64;
    let mut out4 = 0u64;

    x44444444 = a1 & !a2;
    x0F0FF0F0 = a3 ^ a6;
    x4F4FF4F4 = x44444444 | x0F0FF0F0;
    x00FFFF00 = a4 ^ a6;
    x00AAAA00 = x00FFFF00 & !a1;
    x4FE55EF4 = x4F4FF4F4 ^ x00AAAA00;

    x3C3CC3C3 = a2 ^ x0F0FF0F0;
    x3C3C0000 = x3C3CC3C3 & !a6;
    x7373F4F4 = x4F4FF4F4 ^ x3C3C0000;
    x0C840A00 = x4FE55EF4 & !x7373F4F4;

    x00005EF4 = a6 & x4FE55EF4;
    x00FF5EFF = a4 | x00005EF4;
    x00555455 = a1 & x00FF5EFF;
    x3C699796 = x3C3CC3C3 ^ x00555455;
    x30 = x4FE55EF4 & !a5;
    x31 = x30 ^ x3C699796;
    out4 ^= x31;

    x000FF000 = x0F0FF0F0 & x00FFFF00;
    x55AA55AA = a1 ^ a4;
    x26D9A15E = x7373F4F4 ^ x55AA55AA;
    x2FDFAF5F = a3 | x26D9A15E;
    x2FD00F5F = x2FDFAF5F & !x000FF000;

    x55AAFFAA = x00AAAA00 | x55AA55AA;
    x28410014 = x3C699796 & !x55AAFFAA;
    x000000FF = a4 & a6;
    x000000CC = x000000FF & !a2;
    x284100D8 = x28410014 ^ x000000CC;

    x204100D0 = x7373F4F4 & x284100D8;
    x3C3CC3FF = x3C3CC3C3 | x000000FF;
    x1C3CC32F = x3C3CC3FF & !x204100D0;
    x4969967A = a1 ^ x1C3CC32F;
    x10 = x2FD00F5F & a5;
    x11 = x10 ^ x4969967A;
    out2 ^= x11;

    x4CC44CC4 = x4FE55EF4 & !a2;
    x40C040C0 = x4CC44CC4 & !a3;
    xC3C33C3C = !x3C3CC3C3;
    x9669C396 = x55AAFFAA ^ xC3C33C3C;
    xD6A98356 = x40C040C0 ^ x9669C396;
    x00 = a5 & !x0C840A00;
    x01 = x00 ^ xD6A98356;
    out1 ^= x01;

    xD6E9C3D6 = x40C040C0 | x9669C396;
    x4CEEEEC4 = x00AAAA00 | x4CC44CC4;
    x9A072D12 = xD6E9C3D6 ^ x4CEEEEC4;
    x001A000B = a4 & !x4FE55EF4;
    x9A1F2D1B = x9A072D12 | x001A000B;
    x20 = a5 & !x284100D8;
    x21 = x20 ^ x9A1F2D1B;
    out3 ^= x21;
    (out1, out2, out3, out4)
}

pub fn s4(a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a6: u64) -> (u64, u64, u64, u64) {
    let mut x5A5A5A5A: u64;
    let mut x0F0FF0F0: u64;
    let mut x33FF33FF: u64;
    let mut x33FFCC00: u64;
    let mut x0C0030F0: u64;
    let mut x0C0CC0C0: u64;
    let mut x0CF3C03F: u64;
    let mut x5EFBDA7F: u64;
    let mut x52FBCA0F: u64;
    let mut x61C8F93C: u64;
    let mut x00C0C03C: u64;
    let mut x0F0F30C0: u64;
    let mut x3B92A366: u64;
    let mut x30908326: u64;
    let mut x3C90B3D6: u64;
    let mut x33CC33CC: u64;
    let mut x0C0CFFFF: u64;
    let mut x379E5C99: u64;
    let mut x04124C11: u64;
    let mut x56E9861E: u64;
    let mut xA91679E1: u64;
    let mut x9586CA37: u64;
    let mut x8402C833: u64;
    let mut x84C2C83F: u64;
    let mut xB35C94A6: u64;
    let mut x00: u64;
    let mut x01: u64;
    let mut x10: u64;
    let mut x11: u64;
    let mut x20: u64;
    let mut x21: u64;
    let mut x30: u64;
    let mut x31: u64;

    let mut out1: u64 = 0;
    let mut out2: u64 = 0;
    let mut out3: u64 = 0;
    let mut out4: u64 = 0;

    x5A5A5A5A = a1 ^ a3;
    x0F0FF0F0 = a3 ^ a5;
    x33FF33FF = a2 | a4;
    x33FFCC00 = a5 ^ x33FF33FF;
    x0C0030F0 = x0F0FF0F0 & !x33FFCC00;
    x0C0CC0C0 = x0F0FF0F0 & !a2;
    x0CF3C03F = a4 ^ x0C0CC0C0;
    x5EFBDA7F = x5A5A5A5A | x0CF3C03F;
    x52FBCA0F = x5EFBDA7F & !x0C0030F0;
    x61C8F93C = a2 ^ x52FBCA0F;

    x00C0C03C = x0CF3C03F & x61C8F93C;
    x0F0F30C0 = x0F0FF0F0 & !x00C0C03C;
    x3B92A366 = x5A5A5A5A ^ x61C8F93C;
    x30908326 = x3B92A366 & !x0F0F30C0;
    x3C90B3D6 = x0C0030F0 ^ x30908326;

    x33CC33CC = a2 ^ a4;
    x0C0CFFFF = a5 | x0C0CC0C0;
    x379E5C99 = x3B92A366 ^ x0C0CFFFF;
    x04124C11 = x379E5C99 & !x33CC33CC;
    x56E9861E = x52FBCA0F ^ x04124C11;
    x00 = a6 & !x3C90B3D6;
    x01 = x00 ^ x56E9861E;
    out1 ^= x01;

    xA91679E1 = !x56E9861E;
    x10 = x3C90B3D6 & !a6;
    x11 = x10 ^ xA91679E1;
    out2 ^= x11;

    x9586CA37 = x3C90B3D6 ^ xA91679E1;
    x8402C833 = x9586CA37 & !x33CC33CC;
    x84C2C83F = x00C0C03C | x8402C833;
    xB35C94A6 = x379E5C99 ^ x84C2C83F;
    x20 = x61C8F93C | a6;
    x21 = x20 ^ xB35C94A6;
    out3 ^= x21;

    x30 = a6 & x61C8F93C;
    x31 = x30 ^ xB35C94A6;
    out4 ^= x31;

    (out1, out2, out3, out4)
}

pub fn s5(a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a6: u64) -> (u64, u64, u64, u64) {
    let mut x77777777: u64;
    let mut x77770000: u64;
    let mut x22225555: u64;
    let mut x11116666: u64;
    let mut x1F1F6F6F: u64;
    let mut x70700000: u64;
    let mut x43433333: u64;
    let mut x00430033: u64;
    let mut x55557777: u64;
    let mut x55167744: u64;
    let mut x5A19784B: u64;
    let mut x5A1987B4: u64;
    let mut x7A3BD7F5: u64;
    let mut x003B00F5: u64;
    let mut x221955A0: u64;
    let mut x05050707: u64;
    let mut x271C52A7: u64;
    let mut x2A2A82A0: u64;
    let mut x6969B193: u64;
    let mut x1FE06F90: u64;
    let mut x16804E00: u64;
    let mut xE97FB1FF: u64;
    let mut x43403302: u64;
    let mut x35CAED30: u64;
    let mut x37DEFFB7: u64;
    let mut x349ECCB5: u64;
    let mut x0B01234A: u64;
    let mut x101884B4: u64;
    let mut x0FF8EB24: u64;
    let mut x41413333: u64;
    let mut x4FF9FB37: u64;
    let mut x4FC2FBC2: u64;
    let mut x22222222: u64;
    let mut x16BCEE97: u64;
    let mut x0F080B04: u64;
    let mut x19B4E593: u64;
    let mut x5C5C5C5C: u64;
    let mut x4448184C: u64;
    let mut x2DDABE71: u64;
    let mut x6992A63D: u64;
    let mut x00: u64;
    let mut x01: u64;
    let mut x10: u64;
    let mut x11: u64;
    let mut x20: u64;
    let mut x21: u64;
    let mut x30: u64;
    let mut x31: u64;

    let mut out1: u64 = 0;
    let mut out2: u64 = 0;
    let mut out3: u64 = 0;
    let mut out4: u64 = 0;

    x77777777 = a1 | a3;
    x77770000 = x77777777 & !a6;
    x22225555 = a1 ^ x77770000;
    x11116666 = a3 ^ x22225555;
    x1F1F6F6F = a4 | x11116666;

    x70700000 = x77770000 & !a4;
    x43433333 = a3 ^ x70700000;
    x00430033 = a5 & x43433333;
    x55557777 = a1 | x11116666;
    x55167744 = x00430033 ^ x55557777;
    x5A19784B = a4 ^ x55167744;

    x5A1987B4 = a6 ^ x5A19784B;
    x7A3BD7F5 = x22225555 | x5A1987B4;
    x003B00F5 = a5 & x7A3BD7F5;
    x221955A0 = x22225555 ^ x003B00F5;
    x05050707 = a4 & x55557777;
    x271C52A7 = x221955A0 ^ x05050707;

    x2A2A82A0 = x7A3BD7F5 & !a1;
    x6969B193 = x43433333 ^ x2A2A82A0;
    x1FE06F90 = a5 ^ x1F1F6F6F;
    x16804E00 = x1FE06F90 & !x6969B193;
    xE97FB1FF = !x16804E00;
    x20 = xE97FB1FF & !a2;
    x21 = x20 ^ x5A19784B;
    out3 ^= x21;

    x43403302 = x43433333 & !x003B00F5;
    x35CAED30 = x2A2A82A0 ^ x1FE06F90;
    x37DEFFB7 = x271C52A7 | x35CAED30;
    x349ECCB5 = x37DEFFB7 & !x43403302;
    x0B01234A = x1F1F6F6F & !x349ECCB5;

    x101884B4 = x5A1987B4 & x349ECCB5;
    x0FF8EB24 = x1FE06F90 ^ x101884B4;
    x41413333 = x43433333 & x55557777;
    x4FF9FB37 = x0FF8EB24 | x41413333;
    x4FC2FBC2 = x003B00F5 ^ x4FF9FB37;
    x30 = x4FC2FBC2 & a2;
    x31 = x30 ^ x271C52A7;
    out4 ^= x31;

    x22222222 = a1 ^ x77777777;
    x16BCEE97 = x349ECCB5 ^ x22222222;
    x0F080B04 = a4 & x0FF8EB24;
    x19B4E593 = x16BCEE97 ^ x0F080B04;
    x00 = x0B01234A | a2;
    x01 = x00 ^ x19B4E593;
    out1 ^= x01;

    x5C5C5C5C = x1F1F6F6F ^ x43433333;
    x4448184C = x5C5C5C5C & !x19B4E593;
    x2DDABE71 = x22225555 ^ x0FF8EB24;
    x6992A63D = x4448184C ^ x2DDABE71;
    x10 = x1F1F6F6F & a2;
    x11 = x10 ^ x6992A63D;
    out2 ^= x11;

    (out1, out2, out3, out4)
}

pub fn s6(a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a6: u64) -> (u64, u64, u64, u64) {
    let mut x33CC33CC: u64;
    let mut x3333FFFF: u64;
    let mut x11115555: u64;
    let mut x22DD6699: u64;
    let mut x22DD9966: u64;
    let mut x00220099: u64;
    let mut x00551144: u64;
    let mut x33662277: u64;
    let mut x5A5A5A5A: u64;
    let mut x7B7E7A7F: u64;
    let mut x59A31CE6: u64;
    let mut x09030C06: u64;
    let mut x09030000: u64;
    let mut x336622FF: u64;
    let mut x3A6522FF: u64;
    let mut x484D494C: u64;
    let mut x0000B6B3: u64;
    let mut x0F0FB9BC: u64;
    let mut x00FC00F9: u64;
    let mut x0FFFB9FD: u64;
    let mut x5DF75DF7: u64;
    let mut x116600F7: u64;
    let mut x1E69B94B: u64;
    let mut x1668B94B: u64;
    let mut x7B7B7B7B: u64;
    let mut x411E5984: u64;
    let mut x1FFFFDFD: u64;
    let mut x5EE1A479: u64;
    let mut x3CB4DFD2: u64;
    let mut x004B002D: u64;
    let mut xB7B2B6B3: u64;
    let mut xCCC9CDC8: u64;
    let mut xCC82CDE5: u64;
    let mut x0055EEBB: u64;
    let mut x5A5AECE9: u64;
    let mut x0050ECA9: u64;
    let mut xC5CAC1CE: u64;
    let mut xC59A2D67: u64;
    let mut x00: u64;
    let mut x01: u64;
    let mut x10: u64;
    let mut x11: u64;
    let mut x20: u64;
    let mut x21: u64;
    let mut x30: u64;
    let mut x31: u64;

    let mut out1: u64 = 0;
    let mut out2: u64 = 0;
    let mut out3: u64 = 0;
    let mut out4: u64 = 0;

    x33CC33CC = a2 ^ a5;

    x3333FFFF = a2 | a6;
    x11115555 = a1 & x3333FFFF;
    x22DD6699 = x33CC33CC ^ x11115555;
    x22DD9966 = a6 ^ x22DD6699;
    x00220099 = a5 & !x22DD9966;

    x00551144 = a1 & x22DD9966;
    x33662277 = a2 ^ x00551144;
    x5A5A5A5A = a1 ^ a3;
    x7B7E7A7F = x33662277 | x5A5A5A5A;
    x59A31CE6 = x22DD6699 ^ x7B7E7A7F;

    x09030C06 = a3 & x59A31CE6;
    x09030000 = x09030C06 & !a6;
    x336622FF = x00220099 | x33662277;
    x3A6522FF = x09030000 ^ x336622FF;
    x30 = x3A6522FF & a4;
    x31 = x30 ^ x59A31CE6;
    out4 ^= x31;

    x484D494C = a2 ^ x7B7E7A7F;
    x0000B6B3 = a6 & !x484D494C;
    x0F0FB9BC = a3 ^ x0000B6B3;
    x00FC00F9 = a5 & !x09030C06;
    x0FFFB9FD = x0F0FB9BC | x00FC00F9;

    x5DF75DF7 = a1 | x59A31CE6;
    x116600F7 = x336622FF & x5DF75DF7;
    x1E69B94B = x0F0FB9BC ^ x116600F7;
    x1668B94B = x1E69B94B & !x09030000;
    x20 = x00220099 | a4;
    x21 = x20 ^ x1668B94B;
    out3 ^= x21;

    x7B7B7B7B = a2 | x5A5A5A5A;
    x411E5984 = x3A6522FF ^ x7B7B7B7B;
    x1FFFFDFD = x11115555 | x0FFFB9FD;
    x5EE1A479 = x411E5984 ^ x1FFFFDFD;

    x3CB4DFD2 = x22DD6699 ^ x1E69B94B;
    x004B002D = a5 & !x3CB4DFD2;
    xB7B2B6B3 = !x484D494C;
    xCCC9CDC8 = x7B7B7B7B ^ xB7B2B6B3;
    xCC82CDE5 = x004B002D ^ xCCC9CDC8;
    x10 = xCC82CDE5 & !a4;
    x11 = x10 ^ x5EE1A479;
    out2 ^= x11;

    x0055EEBB = a6 ^ x00551144;
    x5A5AECE9 = a1 ^ x0F0FB9BC;
    x0050ECA9 = x0055EEBB & x5A5AECE9;
    xC5CAC1CE = x09030C06 ^ xCCC9CDC8;
    xC59A2D67 = x0050ECA9 ^ xC5CAC1CE;
    x00 = x0FFFB9FD & !a4;
    x01 = x00 ^ xC59A2D67;
    out1 ^= x01;

    (out1, out2, out3, out4)
}

pub fn s7(a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a6: u64) -> (u64, u64, u64, u64) {
    let mut x0FF00FF0: u64;
    let mut x3CC33CC3: u64;
    let mut x00003CC3: u64;
    let mut x0F000F00: u64;
    let mut x5A555A55: u64;
    let mut x00001841: u64;
    let mut x00000F00: u64;
    let mut x33333C33: u64;
    let mut x7B777E77: u64;
    let mut x0FF0F00F: u64;
    let mut x74878E78: u64;
    let mut x003C003C: u64;
    let mut x5A7D5A7D: u64;
    let mut x333300F0: u64;
    let mut x694E5A8D: u64;
    let mut x0FF0CCCC: u64;
    let mut x000F0303: u64;
    let mut x5A505854: u64;
    let mut x33CC000F: u64;
    let mut x699C585B: u64;
    let mut x7F878F78: u64;
    let mut x21101013: u64;
    let mut x7F979F7B: u64;
    let mut x30030CC0: u64;
    let mut x4F9493BB: u64;
    let mut x6F9CDBFB: u64;
    let mut x0000DBFB: u64;
    let mut x00005151: u64;
    let mut x26DAC936: u64;
    let mut x26DA9867: u64;
    let mut x27DA9877: u64;
    let mut x27DA438C: u64;
    let mut x2625C9C9: u64;
    let mut x27FFCBCD: u64;
    let mut x27FF1036: u64;
    let mut x27FF103E: u64;
    let mut xB06B6C44: u64;
    let mut x97947C7A: u64;
    let mut x00: u64;
    let mut x01: u64;
    let mut x10: u64;
    let mut x11: u64;
    let mut x20: u64;
    let mut x21: u64;
    let mut x30: u64;
    let mut x31: u64;

    let mut out1: u64 = 0;
    let mut out2: u64 = 0;
    let mut out3: u64 = 0;
    let mut out4: u64 = 0;

    x0FF00FF0 = a4 ^ a5;
    x3CC33CC3 = a3 ^ x0FF00FF0;
    x00003CC3 = a6 & x3CC33CC3;
    x0F000F00 = a4 & x0FF00FF0;
    x5A555A55 = a2 ^ x0F000F00;
    x00001841 = x00003CC3 & x5A555A55;

    x00000F00 = a6 & x0F000F00;
    x33333C33 = a3 ^ x00000F00;
    x7B777E77 = x5A555A55 | x33333C33;
    x0FF0F00F = a6 ^ x0FF00FF0;
    x74878E78 = x7B777E77 ^ x0FF0F00F;
    x30 = a1 & !x00001841;
    x31 = x30 ^ x74878E78;
    out4 ^= x31;

    x003C003C = a5 & !x3CC33CC3;
    x5A7D5A7D = x5A555A55 | x003C003C;
    x333300F0 = x00003CC3 ^ x33333C33;
    x694E5A8D = x5A7D5A7D ^ x333300F0;

    x0FF0CCCC = x00003CC3 ^ x0FF0F00F;
    x000F0303 = a4 & !x0FF0CCCC;
    x5A505854 = x5A555A55 & !x000F0303;
    x33CC000F = a5 ^ x333300F0;
    x699C585B = x5A505854 ^ x33CC000F;

    x7F878F78 = x0F000F00 | x74878E78;
    x21101013 = a3 & x699C585B;
    x7F979F7B = x7F878F78 | x21101013;
    x30030CC0 = x3CC33CC3 & !x0FF0F00F;
    x4F9493BB = x7F979F7B ^ x30030CC0;
    x00 = x4F9493BB & !a1;
    x01 = x00 ^ x694E5A8D;
    out1 ^= x01;

    x6F9CDBFB = x699C585B | x4F9493BB;
    x0000DBFB = a6 & x6F9CDBFB;
    x00005151 = a2 & x0000DBFB;
    x26DAC936 = x694E5A8D ^ x4F9493BB;
    x26DA9867 = x00005151 ^ x26DAC936;

    x27DA9877 = x21101013 | x26DA9867;
    x27DA438C = x0000DBFB ^ x27DA9877;
    x2625C9C9 = a5 ^ x26DAC936;
    x27FFCBCD = x27DA438C | x2625C9C9;
    x20 = x27FFCBCD & a1;
    x21 = x20 ^ x699C585B;
    out3 ^= x21;

    x27FF1036 = x0000DBFB ^ x27FFCBCD;
    x27FF103E = x003C003C | x27FF1036;
    xB06B6C44 = !x4F9493BB;
    x97947C7A = x27FF103E ^ xB06B6C44;
    x10 = x97947C7A & !a1;
    x11 = x10 ^ x26DA9867;
    out2 ^= x11;

    (out1, out2, out3, out4)
}

pub fn s8(a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a6: u64) -> (u64, u64, u64, u64) {
    let mut x0C0C0C0C: u64;
    let mut x0000F0F0: u64;
    let mut x00FFF00F: u64;
    let mut x00555005: u64;
    let mut x00515001: u64;
    let mut x33000330: u64;
    let mut x77555775: u64;
    let mut x30303030: u64;
    let mut x3030CFCF: u64;
    let mut x30104745: u64;
    let mut x30555745: u64;
    let mut xFF000FF0: u64;
    let mut xCF1048B5: u64;
    let mut x080A080A: u64;
    let mut xC71A40BF: u64;
    let mut xCB164CB3: u64;
    let mut x9E4319E6: u64;
    let mut x000019E6: u64;
    let mut xF429738C: u64;
    let mut xF4296A6A: u64;
    let mut xC729695A: u64;
    let mut xC47C3D2F: u64;
    let mut xF77F3F3F: u64;
    let mut x9E43E619: u64;
    let mut x693CD926: u64;
    let mut xF719A695: u64;
    let mut xF4FF73FF: u64;
    let mut x03E6D56A: u64;
    let mut x56B3803F: u64;
    let mut xF700A600: u64;
    let mut x61008000: u64;
    let mut x03B7856B: u64;
    let mut x62B7056B: u64;
    let mut x00: u64;
    let mut x01: u64;
    let mut x10: u64;
    let mut x11: u64;
    let mut x20: u64;
    let mut x21: u64;
    let mut x30: u64;
    let mut x31: u64;

    let mut out1: u64 = 0;
    let mut out2: u64 = 0;
    let mut out3: u64 = 0;
    let mut out4: u64 = 0;

    x0C0C0C0C = a3 & !a2;
    x0000F0F0 = a5 & !a3;
    x00FFF00F = a4 ^ x0000F0F0;
    x00555005 = a1 & x00FFF00F;
    x00515001 = x00555005 & !x0C0C0C0C;

    x33000330 = a2 & !x00FFF00F;
    x77555775 = a1 | x33000330;
    x30303030 = a2 & !a3;
    x3030CFCF = a5 ^ x30303030;
    x30104745 = x77555775 & x3030CFCF;
    x30555745 = x00555005 | x30104745;

    xFF000FF0 = !x00FFF00F;
    xCF1048B5 = x30104745 ^ xFF000FF0;
    x080A080A = a3 & !x77555775;
    xC71A40BF = xCF1048B5 ^ x080A080A;
    xCB164CB3 = x0C0C0C0C ^ xC71A40BF;
    x10 = x00515001 | a6;
    x11 = x10 ^ xCB164CB3;
    out2 ^= x11;

    x9E4319E6 = a1 ^ xCB164CB3;
    x000019E6 = a5 & x9E4319E6;
    xF429738C = a2 ^ xC71A40BF;
    xF4296A6A = x000019E6 ^ xF429738C;
    xC729695A = x33000330 ^ xF4296A6A;

    xC47C3D2F = x30555745 ^ xF4296A6A;
    xF77F3F3F = a2 | xC47C3D2F;
    x9E43E619 = a5 ^ x9E4319E6;
    x693CD926 = xF77F3F3F ^ x9E43E619;
    x20 = x30555745 & a6;
    x21 = x20 ^ x693CD926;
    out3 ^= x21;

    xF719A695 = x3030CFCF ^ xC729695A;
    xF4FF73FF = a4 | xF429738C;
    x03E6D56A = xF719A695 ^ xF4FF73FF;
    x56B3803F = a1 ^ x03E6D56A;
    x30 = x56B3803F & a6;
    x31 = x30 ^ xC729695A;
    out4 ^= x31;

    xF700A600 = xF719A695 & !a4;
    x61008000 = x693CD926 & xF700A600;
    x03B7856B = x00515001 ^ x03E6D56A;
    x62B7056B = x61008000 ^ x03B7856B;
    x00 = x62B7056B | a6;
    x01 = x00 ^ xC729695A;
    out1 ^= x01;

    (out1, out2, out3, out4)
}
