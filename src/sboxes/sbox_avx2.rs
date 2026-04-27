use std::arch::x86_64::{
    __m256i, _mm256_and_si256, _mm256_andnot_si256, _mm256_cmpeq_epi64, _mm256_or_si256,
    _mm256_set1_epi64x, _mm256_xor_si256, _mm512_andnot_si512,
};

#[inline(always)]
unsafe fn and_not(not_this: __m256i, and_this: __m256i) -> __m256i {
    _mm256_andnot_si256(not_this, and_this)
}

#[inline(always)]
pub unsafe fn s1_avx_2(
    a1: __m256i,
    a2: __m256i,
    a3: __m256i,
    a4: __m256i,
    a5: __m256i,
    a6: __m256i,
) -> (__m256i, __m256i, __m256i, __m256i) {
    let x55005500 = and_not(a5, a1);
    let x5A0F5A0F = _mm256_xor_si256(a4, x55005500);
    let x3333FFFF = _mm256_or_si256(a3, a6);
    let x66666666 = _mm256_xor_si256(a1, a3);
    let x22226666 = _mm256_and_si256(x3333FFFF, x66666666);
    let x2D2D6969 = _mm256_xor_si256(a4, x22226666);
    let x25202160 = and_not(x5A0F5A0F, x2D2D6969);

    let x00FFFF00 = _mm256_xor_si256(a5, a6);
    let x33CCCC33 = _mm256_xor_si256(a3, x00FFFF00);
    let x4803120C = and_not(x33CCCC33, x5A0F5A0F);
    let x2222FFFF = _mm256_or_si256(a6, x22226666);
    let x6A21EDF3 = _mm256_xor_si256(x4803120C, x2222FFFF);
    let x4A01CC93 = and_not(x25202160, x6A21EDF3);

    let x5555FFFF = _mm256_or_si256(a1, a6);
    let x7F75FFFF = _mm256_or_si256(x6A21EDF3, x5555FFFF);
    let x00D20096 = and_not(x2D2D6969, a5);
    let x7FA7FF69 = _mm256_xor_si256(x7F75FFFF, x00D20096);

    let x0A0A0000 = and_not(x5555FFFF, a4);
    let x0AD80096 = _mm256_xor_si256(x00D20096, x0A0A0000);
    let x00999900 = and_not(x66666666, x00FFFF00);
    let x0AD99996 = _mm256_or_si256(x0AD80096, x00999900);

    let x22332233 = and_not(x55005500, a3);
    let x257AA5F0 = _mm256_xor_si256(x5A0F5A0F, x7F75FFFF);
    let x054885C0 = and_not(x22332233, x257AA5F0);
    let ones = _mm256_set1_epi64x(-1i64);
    let xFAB77A3F = _mm256_xor_si256(x054885C0, ones);
    let x2221EDF3 = _mm256_and_si256(x3333FFFF, x6A21EDF3);
    let xD89697CC = _mm256_xor_si256(xFAB77A3F, x2221EDF3);
    let out3 = _mm256_xor_si256(and_not(a2, x7FA7FF69), xD89697CC);

    let x05B77AC0 = _mm256_xor_si256(x00FFFF00, x054885C0);
    let x05F77AD6 = _mm256_or_si256(x00D20096, x05B77AC0);
    let x36C48529 = _mm256_xor_si256(x3333FFFF, x05F77AD6);
    let x6391D07C = _mm256_xor_si256(a1, x36C48529);
    let xBB0747B0 = _mm256_xor_si256(xD89697CC, x6391D07C);
    let out1 = _mm256_xor_si256(_mm256_or_si256(x25202160, a2), xBB0747B0);

    let x4C460000 = _mm256_xor_si256(x3333FFFF, x7F75FFFF);
    let x4EDF9996 = _mm256_or_si256(x0AD99996, x4C460000);
    let x2D4E49EA = _mm256_xor_si256(x6391D07C, x4EDF9996);
    let xBBFFFFB0 = _mm256_or_si256(x00FFFF00, xBB0747B0);
    let x96B1B65A = _mm256_xor_si256(x2D4E49EA, xBBFFFFB0);
    let out2 = _mm256_xor_si256(_mm256_or_si256(x4A01CC93, a2), x96B1B65A);

    let x5AFF5AFF = _mm256_or_si256(a5, x5A0F5A0F);
    let x52B11215 = and_not(x2D4E49EA, x5AFF5AFF);
    let x4201C010 = _mm256_and_si256(x4A01CC93, x6391D07C);
    let x10B0D205 = _mm256_xor_si256(x52B11215, x4201C010);
    let out4 = _mm256_xor_si256(_mm256_or_si256(x10B0D205, a2), x0AD99996);

    (out1, out2, out3, out4)
}

#[inline(always)]
pub unsafe fn s2_avx_2(
    a1: __m256i,
    a2: __m256i,
    a3: __m256i,
    a4: __m256i,
    a5: __m256i,
    a6: __m256i,
) -> (__m256i, __m256i, __m256i, __m256i) {
    let ones = _mm256_set1_epi64x(-1i64);

    let x33CC33CC = _mm256_xor_si256(a2, a5);
    let x55550000 = and_not(a6, a1);
    let x00AA00FF = and_not(x55550000, a5);
    let x33BB33FF = _mm256_or_si256(a2, x00AA00FF);
    let x33CC0000 = and_not(a6, x33CC33CC);
    let x11441144 = _mm256_and_si256(a1, x33CC33CC);
    let x11BB11BB = _mm256_xor_si256(a5, x11441144);
    let x003311BB = and_not(x33CC0000, x11BB11BB);
    let x00000F0F = _mm256_and_si256(a3, a6);
    let x336600FF = _mm256_xor_si256(x00AA00FF, x33CC0000);
    let x332200FF = _mm256_and_si256(x33BB33FF, x336600FF);
    let x332200F0 = and_not(x00000F0F, x332200FF);
    let x0302000F = _mm256_and_si256(a3, x332200FF);
    let xAAAAAAAA = _mm256_xor_si256(a1, ones);
    let xA9A8AAA5 = _mm256_xor_si256(x0302000F, xAAAAAAAA);
    let x33CCCC33 = _mm256_xor_si256(a6, x33CC33CC);
    let x33CCC030 = and_not(x00000F0F, x33CCCC33);
    let x9A646A95 = _mm256_xor_si256(xA9A8AAA5, x33CCC030);
    let x10 = and_not(x332200F0, a4);
    let out2 = _mm256_xor_si256(x10, x9A646A95);
    let x00333303 = and_not(x33CCC030, a2);
    let x118822B8 = _mm256_xor_si256(x11BB11BB, x00333303);
    let xA8208805 = and_not(x118822B8, xA9A8AAA5);
    let x3CC3C33C = _mm256_xor_si256(a3, x33CCCC33);
    let x94E34B39 = _mm256_xor_si256(xA8208805, x3CC3C33C);
    let x00 = and_not(a4, x33BB33FF);
    let out1 = _mm256_xor_si256(x00, x94E34B39);
    let x0331330C = _mm256_xor_si256(x0302000F, x00333303);
    let x3FF3F33C = _mm256_or_si256(x3CC3C33C, x0331330C);
    let xA9DF596A = _mm256_xor_si256(x33BB33FF, x9A646A95);
    let xA9DF5F6F = _mm256_or_si256(x00000F0F, xA9DF596A);
    let x962CAC53 = _mm256_xor_si256(x3FF3F33C, xA9DF5F6F);
    let xA9466A6A = _mm256_xor_si256(x332200FF, x9A646A95);
    let x3DA52153 = _mm256_xor_si256(x94E34B39, xA9466A6A);
    let x29850143 = _mm256_and_si256(xA9DF5F6F, x3DA52153);
    let x33C0330C = _mm256_and_si256(x33CC33CC, x3FF3F33C);
    let x1A45324F = _mm256_xor_si256(x29850143, x33C0330C);
    let x20 = _mm256_or_si256(x1A45324F, a4);
    let out3 = _mm256_xor_si256(x20, x962CAC53);
    let x0A451047 = and_not(x118822B8, x1A45324F);
    let xBBDFDD7B = _mm256_or_si256(x33CCCC33, xA9DF596A);
    let xB19ACD3C = _mm256_xor_si256(x0A451047, xBBDFDD7B);
    let x30 = _mm256_or_si256(x003311BB, a4);
    let out4 = _mm256_xor_si256(x30, xB19ACD3C);

    (out1, out2, out3, out4)
}

#[inline(always)]
pub unsafe fn s3_avx_2(
    a1: __m256i,
    a2: __m256i,
    a3: __m256i,
    a4: __m256i,
    a5: __m256i,
    a6: __m256i,
) -> (__m256i, __m256i, __m256i, __m256i) {
    let ones = _mm256_set1_epi64x(-1i64);

    let x44444444 = and_not(a2, a1);
    let x0F0FF0F0 = _mm256_xor_si256(a3, a6);
    let x4F4FF4F4 = _mm256_or_si256(x44444444, x0F0FF0F0);
    let x00FFFF00 = _mm256_xor_si256(a4, a6);
    let x00AAAA00 = and_not(a1, x00FFFF00);
    let x4FE55EF4 = _mm256_xor_si256(x4F4FF4F4, x00AAAA00);

    let x3C3CC3C3 = _mm256_xor_si256(a2, x0F0FF0F0);
    let x3C3C0000 = and_not(a6, x3C3CC3C3);
    let x7373F4F4 = _mm256_xor_si256(x4F4FF4F4, x3C3C0000);
    let x0C840A00 = and_not(x7373F4F4, x4FE55EF4);

    let x00005EF4 = _mm256_and_si256(a6, x4FE55EF4);
    let x00FF5EFF = _mm256_or_si256(a4, x00005EF4);
    let x00555455 = _mm256_and_si256(a1, x00FF5EFF);
    let x3C699796 = _mm256_xor_si256(x3C3CC3C3, x00555455);
    let x30 = and_not(a5, x4FE55EF4);
    let out4 = _mm256_xor_si256(x30, x3C699796);

    let x000FF000 = _mm256_and_si256(x0F0FF0F0, x00FFFF00);
    let x55AA55AA = _mm256_xor_si256(a1, a4);
    let x26D9A15E = _mm256_xor_si256(x7373F4F4, x55AA55AA);
    let x2FDFAF5F = _mm256_or_si256(a3, x26D9A15E);
    let x2FD00F5F = and_not(x000FF000, x2FDFAF5F);

    let x55AAFFAA = _mm256_or_si256(x00AAAA00, x55AA55AA);
    let x28410014 = and_not(x55AAFFAA, x3C699796);
    let x000000FF = _mm256_and_si256(a4, a6);
    let x000000CC = and_not(a2, x000000FF);
    let x284100D8 = _mm256_xor_si256(x28410014, x000000CC);

    let x204100D0 = _mm256_and_si256(x7373F4F4, x284100D8);
    let x3C3CC3FF = _mm256_or_si256(x3C3CC3C3, x000000FF);
    let x1C3CC32F = and_not(x204100D0, x3C3CC3FF);
    let x4969967A = _mm256_xor_si256(a1, x1C3CC32F);
    let x10 = _mm256_and_si256(x2FD00F5F, a5);
    let out2 = _mm256_xor_si256(x10, x4969967A);

    let x4CC44CC4 = and_not(a2, x4FE55EF4);
    let x40C040C0 = and_not(a3, x4CC44CC4);
    let xC3C33C3C = _mm256_xor_si256(x3C3CC3C3, ones);
    let x9669C396 = _mm256_xor_si256(x55AAFFAA, xC3C33C3C);
    let xD6A98356 = _mm256_xor_si256(x40C040C0, x9669C396);
    let x00 = and_not(x0C840A00, a5);
    let out1 = _mm256_xor_si256(x00, xD6A98356);

    let xD6E9C3D6 = _mm256_or_si256(x40C040C0, x9669C396);
    let x4CEEEEC4 = _mm256_or_si256(x00AAAA00, x4CC44CC4);
    let x9A072D12 = _mm256_xor_si256(xD6E9C3D6, x4CEEEEC4);
    let x001A000B = and_not(x4FE55EF4, a4);
    let x9A1F2D1B = _mm256_or_si256(x9A072D12, x001A000B);
    let x20 = and_not(x284100D8, a5);
    let out3 = _mm256_xor_si256(x20, x9A1F2D1B);

    (out1, out2, out3, out4)
}

#[inline(always)]
pub unsafe fn s4_avx_2(
    a1: __m256i,
    a2: __m256i,
    a3: __m256i,
    a4: __m256i,
    a5: __m256i,
    a6: __m256i,
) -> (__m256i, __m256i, __m256i, __m256i) {
    let ones = _mm256_set1_epi64x(-1i64);

    let x5A5A5A5A = _mm256_xor_si256(a1, a3);
    let x0F0FF0F0 = _mm256_xor_si256(a3, a5);
    let x33FF33FF = _mm256_or_si256(a2, a4);
    let x33FFCC00 = _mm256_xor_si256(a5, x33FF33FF);
    let x0C0030F0 = and_not(x33FFCC00, x0F0FF0F0);
    let x0C0CC0C0 = and_not(a2, x0F0FF0F0);
    let x0CF3C03F = _mm256_xor_si256(a4, x0C0CC0C0);
    let x5EFBDA7F = _mm256_or_si256(x5A5A5A5A, x0CF3C03F);
    let x52FBCA0F = and_not(x0C0030F0, x5EFBDA7F);
    let x61C8F93C = _mm256_xor_si256(a2, x52FBCA0F);

    let x00C0C03C = _mm256_and_si256(x0CF3C03F, x61C8F93C);
    let x0F0F30C0 = and_not(x00C0C03C, x0F0FF0F0);
    let x3B92A366 = _mm256_xor_si256(x5A5A5A5A, x61C8F93C);
    let x30908326 = and_not(x0F0F30C0, x3B92A366);
    let x3C90B3D6 = _mm256_xor_si256(x0C0030F0, x30908326);

    let x33CC33CC = _mm256_xor_si256(a2, a4);
    let x0C0CFFFF = _mm256_or_si256(a5, x0C0CC0C0);
    let x379E5C99 = _mm256_xor_si256(x3B92A366, x0C0CFFFF);
    let x04124C11 = and_not(x33CC33CC, x379E5C99);
    let x56E9861E = _mm256_xor_si256(x52FBCA0F, x04124C11);

    let x00 = and_not(x3C90B3D6, a6);
    let out1 = _mm256_xor_si256(x00, x56E9861E);

    let xA91679E1 = _mm256_xor_si256(x56E9861E, ones);
    let x10 = and_not(a6, x3C90B3D6);
    let out2 = _mm256_xor_si256(x10, xA91679E1);

    let x9586CA37 = _mm256_xor_si256(x3C90B3D6, xA91679E1);
    let x8402C833 = and_not(x33CC33CC, x9586CA37);
    let x84C2C83F = _mm256_or_si256(x00C0C03C, x8402C833);
    let xB35C94A6 = _mm256_xor_si256(x379E5C99, x84C2C83F);
    let x20 = _mm256_or_si256(x61C8F93C, a6);
    let out3 = _mm256_xor_si256(x20, xB35C94A6);

    let x30 = _mm256_and_si256(a6, x61C8F93C);
    let out4 = _mm256_xor_si256(x30, xB35C94A6);

    (out1, out2, out3, out4)
}

#[inline(always)]
pub unsafe fn s5_avx_2(
    a1: __m256i,
    a2: __m256i,
    a3: __m256i,
    a4: __m256i,
    a5: __m256i,
    a6: __m256i,
) -> (__m256i, __m256i, __m256i, __m256i) {
    let ones = _mm256_set1_epi64x(-1i64);

    let x77777777 = _mm256_or_si256(a1, a3);
    let x77770000 = and_not(a6, x77777777);
    let x22225555 = _mm256_xor_si256(a1, x77770000);
    let x11116666 = _mm256_xor_si256(a3, x22225555);
    let x1F1F6F6F = _mm256_or_si256(a4, x11116666);

    let x70700000 = and_not(a4, x77770000);
    let x43433333 = _mm256_xor_si256(a3, x70700000);
    let x00430033 = _mm256_and_si256(a5, x43433333);
    let x55557777 = _mm256_or_si256(a1, x11116666);
    let x55167744 = _mm256_xor_si256(x00430033, x55557777);
    let x5A19784B = _mm256_xor_si256(a4, x55167744);

    let x5A1987B4 = _mm256_xor_si256(a6, x5A19784B);
    let x7A3BD7F5 = _mm256_or_si256(x22225555, x5A1987B4);
    let x003B00F5 = _mm256_and_si256(a5, x7A3BD7F5);
    let x221955A0 = _mm256_xor_si256(x22225555, x003B00F5);
    let x05050707 = _mm256_and_si256(a4, x55557777);
    let x271C52A7 = _mm256_xor_si256(x221955A0, x05050707);

    let x2A2A82A0 = and_not(a1, x7A3BD7F5);
    let x6969B193 = _mm256_xor_si256(x43433333, x2A2A82A0);
    let x1FE06F90 = _mm256_xor_si256(a5, x1F1F6F6F);
    let x16804E00 = and_not(x6969B193, x1FE06F90);
    let xE97FB1FF = _mm256_xor_si256(x16804E00, ones);
    let x20 = and_not(a2, xE97FB1FF);
    let out3 = _mm256_xor_si256(x20, x5A19784B);

    let x43403302 = and_not(x003B00F5, x43433333);
    let x35CAED30 = _mm256_xor_si256(x2A2A82A0, x1FE06F90);
    let x37DEFFB7 = _mm256_or_si256(x271C52A7, x35CAED30);
    let x349ECCB5 = and_not(x43403302, x37DEFFB7);
    let x0B01234A = and_not(x349ECCB5, x1F1F6F6F);

    let x101884B4 = _mm256_and_si256(x5A1987B4, x349ECCB5);
    let x0FF8EB24 = _mm256_xor_si256(x1FE06F90, x101884B4);
    let x41413333 = _mm256_and_si256(x43433333, x55557777);
    let x4FF9FB37 = _mm256_or_si256(x0FF8EB24, x41413333);
    let x4FC2FBC2 = _mm256_xor_si256(x003B00F5, x4FF9FB37);
    let x30 = _mm256_and_si256(x4FC2FBC2, a2);
    let out4 = _mm256_xor_si256(x30, x271C52A7);

    let x22222222 = _mm256_xor_si256(a1, x77777777);
    let x16BCEE97 = _mm256_xor_si256(x349ECCB5, x22222222);
    let x0F080B04 = _mm256_and_si256(a4, x0FF8EB24);
    let x19B4E593 = _mm256_xor_si256(x16BCEE97, x0F080B04);
    let x00 = _mm256_or_si256(x0B01234A, a2);
    let out1 = _mm256_xor_si256(x00, x19B4E593);

    let x5C5C5C5C = _mm256_xor_si256(x1F1F6F6F, x43433333);
    let x4448184C = and_not(x19B4E593, x5C5C5C5C);
    let x2DDABE71 = _mm256_xor_si256(x22225555, x0FF8EB24);
    let x6992A63D = _mm256_xor_si256(x4448184C, x2DDABE71);
    let x10 = _mm256_and_si256(x1F1F6F6F, a2);
    let out2 = _mm256_xor_si256(x10, x6992A63D);

    (out1, out2, out3, out4)
}

#[inline(always)]
pub unsafe fn s6_avx_2(
    a1: __m256i,
    a2: __m256i,
    a3: __m256i,
    a4: __m256i,
    a5: __m256i,
    a6: __m256i,
) -> (__m256i, __m256i, __m256i, __m256i) {
    let ones = _mm256_set1_epi64x(-1i64);

    let x33CC33CC = _mm256_xor_si256(a2, a5);
    let x3333FFFF = _mm256_or_si256(a2, a6);
    let x11115555 = _mm256_and_si256(a1, x3333FFFF);
    let x22DD6699 = _mm256_xor_si256(x33CC33CC, x11115555);
    let x22DD9966 = _mm256_xor_si256(a6, x22DD6699);
    let x00220099 = and_not(x22DD9966, a5);

    let x00551144 = _mm256_and_si256(a1, x22DD9966);
    let x33662277 = _mm256_xor_si256(a2, x00551144);
    let x5A5A5A5A = _mm256_xor_si256(a1, a3);
    let x7B7E7A7F = _mm256_or_si256(x33662277, x5A5A5A5A);
    let x59A31CE6 = _mm256_xor_si256(x22DD6699, x7B7E7A7F);

    let x09030C06 = _mm256_and_si256(a3, x59A31CE6);
    let x09030000 = and_not(a6, x09030C06);
    let x336622FF = _mm256_or_si256(x00220099, x33662277);
    let x3A6522FF = _mm256_xor_si256(x09030000, x336622FF);
    let x30 = _mm256_and_si256(x3A6522FF, a4);
    let out4 = _mm256_xor_si256(x30, x59A31CE6);

    let x484D494C = _mm256_xor_si256(a2, x7B7E7A7F);
    let x0000B6B3 = and_not(x484D494C, a6);
    let x0F0FB9BC = _mm256_xor_si256(a3, x0000B6B3);
    let x00FC00F9 = and_not(x09030C06, a5);
    let x0FFFB9FD = _mm256_or_si256(x0F0FB9BC, x00FC00F9);

    let x5DF75DF7 = _mm256_or_si256(a1, x59A31CE6);
    let x116600F7 = _mm256_and_si256(x336622FF, x5DF75DF7);
    let x1E69B94B = _mm256_xor_si256(x0F0FB9BC, x116600F7);
    let x1668B94B = and_not(x09030000, x1E69B94B);
    let x20 = _mm256_or_si256(x00220099, a4);
    let out3 = _mm256_xor_si256(x20, x1668B94B);

    let x7B7B7B7B = _mm256_or_si256(a2, x5A5A5A5A);
    let x411E5984 = _mm256_xor_si256(x3A6522FF, x7B7B7B7B);
    let x1FFFFDFD = _mm256_or_si256(x11115555, x0FFFB9FD);
    let x5EE1A479 = _mm256_xor_si256(x411E5984, x1FFFFDFD);

    let x3CB4DFD2 = _mm256_xor_si256(x22DD6699, x1E69B94B);
    let x004B002D = and_not(x3CB4DFD2, a5);
    let xB7B2B6B3 = _mm256_xor_si256(x484D494C, ones);
    let xCCC9CDC8 = _mm256_xor_si256(x7B7B7B7B, xB7B2B6B3);
    let xCC82CDE5 = _mm256_xor_si256(x004B002D, xCCC9CDC8);
    let x10 = and_not(a4, xCC82CDE5);
    let out2 = _mm256_xor_si256(x10, x5EE1A479);

    let x0055EEBB = _mm256_xor_si256(a6, x00551144);
    let x5A5AECE9 = _mm256_xor_si256(a1, x0F0FB9BC);
    let x0050ECA9 = _mm256_and_si256(x0055EEBB, x5A5AECE9);
    let xC5CAC1CE = _mm256_xor_si256(x09030C06, xCCC9CDC8);
    let xC59A2D67 = _mm256_xor_si256(x0050ECA9, xC5CAC1CE);
    let x00 = and_not(a4, x0FFFB9FD);
    let out1 = _mm256_xor_si256(x00, xC59A2D67);

    (out1, out2, out3, out4)
}

#[inline(always)]
pub unsafe fn s7_avx_2(
    a1: __m256i,
    a2: __m256i,
    a3: __m256i,
    a4: __m256i,
    a5: __m256i,
    a6: __m256i,
) -> (__m256i, __m256i, __m256i, __m256i) {
    let ones = _mm256_set1_epi64x(-1i64);

    let x0FF00FF0 = _mm256_xor_si256(a4, a5);
    let x3CC33CC3 = _mm256_xor_si256(a3, x0FF00FF0);
    let x00003CC3 = _mm256_and_si256(a6, x3CC33CC3);
    let x0F000F00 = _mm256_and_si256(a4, x0FF00FF0);
    let x5A555A55 = _mm256_xor_si256(a2, x0F000F00);
    let x00001841 = _mm256_and_si256(x00003CC3, x5A555A55);

    let x00000F00 = _mm256_and_si256(a6, x0F000F00);
    let x33333C33 = _mm256_xor_si256(a3, x00000F00);
    let x7B777E77 = _mm256_or_si256(x5A555A55, x33333C33);
    let x0FF0F00F = _mm256_xor_si256(a6, x0FF00FF0);
    let x74878E78 = _mm256_xor_si256(x7B777E77, x0FF0F00F);
    let x30 = and_not(x00001841, a1);
    let out4 = _mm256_xor_si256(x30, x74878E78);

    let x003C003C = and_not(x3CC33CC3, a5);
    let x5A7D5A7D = _mm256_or_si256(x5A555A55, x003C003C);
    let x333300F0 = _mm256_xor_si256(x00003CC3, x33333C33);
    let x694E5A8D = _mm256_xor_si256(x5A7D5A7D, x333300F0);

    let x0FF0CCCC = _mm256_xor_si256(x00003CC3, x0FF0F00F);
    let x000F0303 = and_not(x0FF0CCCC, a4);
    let x5A505854 = and_not(x000F0303, x5A555A55);
    let x33CC000F = _mm256_xor_si256(a5, x333300F0);
    let x699C585B = _mm256_xor_si256(x5A505854, x33CC000F);

    let x7F878F78 = _mm256_or_si256(x0F000F00, x74878E78);
    let x21101013 = _mm256_and_si256(a3, x699C585B);
    let x7F979F7B = _mm256_or_si256(x7F878F78, x21101013);
    let x30030CC0 = and_not(x0FF0F00F, x3CC33CC3);
    let x4F9493BB = _mm256_xor_si256(x7F979F7B, x30030CC0);
    let x00 = and_not(a1, x4F9493BB);
    let out1 = _mm256_xor_si256(x00, x694E5A8D);

    let x6F9CDBFB = _mm256_or_si256(x699C585B, x4F9493BB);
    let x0000DBFB = _mm256_and_si256(a6, x6F9CDBFB);
    let x00005151 = _mm256_and_si256(a2, x0000DBFB);
    let x26DAC936 = _mm256_xor_si256(x694E5A8D, x4F9493BB);
    let x26DA9867 = _mm256_xor_si256(x00005151, x26DAC936);

    let x27DA9877 = _mm256_or_si256(x21101013, x26DA9867);
    let x27DA438C = _mm256_xor_si256(x0000DBFB, x27DA9877);
    let x2625C9C9 = _mm256_xor_si256(a5, x26DAC936);
    let x27FFCBCD = _mm256_or_si256(x27DA438C, x2625C9C9);
    let x20 = _mm256_and_si256(x27FFCBCD, a1);
    let out3 = _mm256_xor_si256(x20, x699C585B);

    let x27FF1036 = _mm256_xor_si256(x0000DBFB, x27FFCBCD);
    let x27FF103E = _mm256_or_si256(x003C003C, x27FF1036);
    let xB06B6C44 = _mm256_xor_si256(x4F9493BB, ones);
    let x97947C7A = _mm256_xor_si256(x27FF103E, xB06B6C44);
    let x10 = and_not(a1, x97947C7A);
    let out2 = _mm256_xor_si256(x10, x26DA9867);

    (out1, out2, out3, out4)
}

#[inline(always)]
pub unsafe fn s8_avx_2(
    a1: __m256i,
    a2: __m256i,
    a3: __m256i,
    a4: __m256i,
    a5: __m256i,
    a6: __m256i,
) -> (__m256i, __m256i, __m256i, __m256i) {
    let ones = _mm256_set1_epi64x(-1i64);

    let x0C0C0C0C = and_not(a2, a3);
    let x0000F0F0 = and_not(a3, a5);
    let x00FFF00F = _mm256_xor_si256(a4, x0000F0F0);
    let x00555005 = _mm256_and_si256(a1, x00FFF00F);
    let x00515001 = and_not(x0C0C0C0C, x00555005);

    let x33000330 = and_not(x00FFF00F, a2);
    let x77555775 = _mm256_or_si256(a1, x33000330);
    let x30303030 = and_not(a3, a2);
    let x3030CFCF = _mm256_xor_si256(a5, x30303030);
    let x30104745 = _mm256_and_si256(x77555775, x3030CFCF);
    let x30555745 = _mm256_or_si256(x00555005, x30104745);

    let xFF000FF0 = _mm256_xor_si256(x00FFF00F, ones);
    let xCF1048B5 = _mm256_xor_si256(x30104745, xFF000FF0);
    let x080A080A = and_not(x77555775, a3);
    let xC71A40BF = _mm256_xor_si256(xCF1048B5, x080A080A);
    let xCB164CB3 = _mm256_xor_si256(x0C0C0C0C, xC71A40BF);
    let x10 = _mm256_or_si256(x00515001, a6);
    let out2 = _mm256_xor_si256(x10, xCB164CB3);

    let x9E4319E6 = _mm256_xor_si256(a1, xCB164CB3);
    let x000019E6 = _mm256_and_si256(a5, x9E4319E6);
    let xF429738C = _mm256_xor_si256(a2, xC71A40BF);
    let xF4296A6A = _mm256_xor_si256(x000019E6, xF429738C);
    let xC729695A = _mm256_xor_si256(x33000330, xF4296A6A);

    let xC47C3D2F = _mm256_xor_si256(x30555745, xF4296A6A);
    let xF77F3F3F = _mm256_or_si256(a2, xC47C3D2F);
    let x9E43E619 = _mm256_xor_si256(a5, x9E4319E6);
    let x693CD926 = _mm256_xor_si256(xF77F3F3F, x9E43E619);
    let x20 = _mm256_and_si256(x30555745, a6);
    let out3 = _mm256_xor_si256(x20, x693CD926);

    let xF719A695 = _mm256_xor_si256(x3030CFCF, xC729695A);
    let xF4FF73FF = _mm256_or_si256(a4, xF429738C);
    let x03E6D56A = _mm256_xor_si256(xF719A695, xF4FF73FF);
    let x56B3803F = _mm256_xor_si256(a1, x03E6D56A);
    let x30 = _mm256_and_si256(x56B3803F, a6);
    let out4 = _mm256_xor_si256(x30, xC729695A);

    let xF700A600 = and_not(a4, xF719A695);
    let x61008000 = _mm256_and_si256(x693CD926, xF700A600);
    let x03B7856B = _mm256_xor_si256(x00515001, x03E6D56A);
    let x62B7056B = _mm256_xor_si256(x61008000, x03B7856B);
    let x00 = _mm256_or_si256(x62B7056B, a6);
    let out1 = _mm256_xor_si256(x00, xC729695A);

    (out1, out2, out3, out4)
}
