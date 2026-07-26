#pragma once

namespace Video {

    // Te dwie linijki naprawi¹ b³¹d kompilacji:
    extern unsigned int screen_width;
    extern unsigned int screen_height;

    // Funkcja do ustawienia dynamicznego adresu przed rysowaniem
    void Init(unsigned int* fb_address, unsigned int w, unsigned int h, unsigned int p);
    // konwertery formatów 
    unsigned int ConvertBGRA(unsigned char r, unsigned char g, unsigned char b, unsigned char a = 0);

    unsigned int ConvertRGBA(unsigned char r, unsigned char g, unsigned char b, unsigned char a = 0);


    // 1. Podstawowy fundament: pojedynczy piksel
    void DrawPixel(unsigned int x, unsigned int y, unsigned int color);

    // 2. Czyszczenie ca³ego ekranu (Twoja ulepszona funkcja ze screena)
    void FillScreen(unsigned int color);

    // 3. Rysowanie prostok¹tów (wype³nionych) - do budowy ramek okien
    void DrawRect(unsigned int x, unsigned int y, unsigned int width, unsigned int height, unsigned int color);

    void WaitVSync();

    // NOWOŒÆ: Funkcja przerzucaj¹ca gotowy bufor z RAM na ekran karty graficznej
    void Refresh();
}