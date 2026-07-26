#include "video.hpp"

namespace Video {
    unsigned int screen_width = 0;
    unsigned int screen_height = 0;
    unsigned int pitch = 0;
    unsigned int* framebuffer = nullptr; // Prawdziwy ekran (hardware)
    unsigned int* backbuffer = nullptr;  // Nasz ukryty bufor w RAM

    void Init(unsigned int* fb_address, unsigned int w, unsigned int h, unsigned int p) {
        framebuffer = fb_address;
        screen_width = w;
        screen_height = h;
        pitch = p;

        // Wskazujemy na bezpieczny adres w RAM (32 MB). QEMU ma 512 MB, wiêc to super bezpieczne miejsce.
        backbuffer = reinterpret_cast<unsigned int*>(0x02000000);
    }

    unsigned int ConvertBGRA(unsigned char r, unsigned char g, unsigned char b, unsigned char a) {
        return (a << 24) | (r << 16) | (g << 8) | b;
    }

    unsigned int ConvertRGBA(unsigned char r, unsigned char g, unsigned char b, unsigned char a) {
        return (a << 24) | (b << 16) | (g << 8) | r;
    }

    void DrawPixel(unsigned int x, unsigned int y, unsigned int color) {
        //if (backbuffer == nullptr) return;
        // Rysujemy do backbuffera!
        if (x >= screen_width || y >= screen_height) return;

        framebuffer[y * (pitch / 4) + x] = color;
    }

    void FillScreen(unsigned int color) {
        if (framebuffer == nullptr) return;

        unsigned int pixels_per_row = pitch / 4;

        for (unsigned int y = 0; y < screen_height; y++)
        {
            for (unsigned int x = 0; x < screen_width; x++)
            {
                framebuffer[y * pixels_per_row + x] = color;
            }
        }
    }

    void DrawRect(unsigned int x, unsigned int y, unsigned int width, unsigned int height, unsigned int color) {
        if (x >= screen_width || y >= screen_height) return;

        // Obcinamy szerokoœæ i wysokoœæ, jeœli rysujemy czêœciowo poza ekranem
        if (x + width > screen_width) width = screen_width - x;
        if (y + height > screen_height) height = screen_height - y;

        for (unsigned int row = 0; row < height; row++) {
            for (unsigned int col = 0; col < width; col++) {
                DrawPixel(x + col, y + row, color);
            }
        }
    }

    void WaitVSync() {
        unsigned char status = 0;

        // 1. Czekaj, a¿ skoñczy siê poprzedni powrót pionowy (jeœli trwa³)
        do {
            __asm__ volatile ("inb %%dx, %%al" : "=a"(status) : "d"(0x3DA));
        } while (status & 8);

        // 2. Czekaj na pocz¹tek nowego powrotu pionowego
        do {
            __asm__ volatile ("inb %%dx, %%al" : "=a"(status) : "d"(0x3DA));
        } while (!(status & 8));
    }

    // Implementacja odœwie¿ania ekranu
    void Refresh() {
        if (framebuffer == nullptr || backbuffer == nullptr) return;

        unsigned int total_pixels = screen_width * screen_height;
        for (unsigned int i = 0; i < total_pixels; i++) {
            framebuffer[i] = backbuffer[i]; // Kopiujemy piksel po pikselu z RAM do karty graficznej
        }
    }
}