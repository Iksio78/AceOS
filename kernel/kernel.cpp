#include "kernel.hpp"
#include "../drivers/video.hpp"
#include "../include/limine.h"

// Pomocnicza funkcja czytaj¹ca takt procesora (RDTSC)
inline unsigned long long GetCPUCycles() {
    unsigned int low, high;
    __asm__ volatile ("rdtsc" : "=a" (low), "=d" (high));
    return ((unsigned long long)high << 32) | low;
}

extern "C" void KernelMain() {

    while (1)
    {
    }
    /*if (magic != 0x2BADB002 || multiboot_structure == nullptr) {
        while (1) { __asm__("hlt"); }
    }*/

    MultibootInfo* mbi = reinterpret_cast<MultibootInfo*>(multiboot_structure);

    //Zainicjalizuj wideo (pomiñmy na sekundê losowe mbi, sprawdŸmy na sztywno)
    if ((flags & (1 << 11)) != 0) {
        unsigned int* fb = reinterpret_cast<unsigned int*>(static_cast<unsigned int>(framebuffer_addr));
        Video::Init(fb, framebuffer_width, framebuffer_height);
    }
    else {
        Video::Init(reinterpret_cast<unsigned int*>(0xFD000000), 1024, 768);
    }

    //Video::Init(reinterpret_cast<unsigned int*>(0xFD000000), 1920, 1080);

    // Przypisz w i h bezpoœrednio z tego, co zapisa³ sterownik!
    unsigned int w = 1920;
    unsigned int h = 1080;

    unsigned int tlo_aceos = Video::ConvertBGRA(51, 153, 255, 255);
    unsigned int szary_okno = Video::ConvertBGRA(192, 192, 192, 255);
    unsigned int pasek_tytulowy = Video::ConvertBGRA(0, 0, 128, 255);

    // Zmienne do pomiaru wydajnoœci i animacji
    unsigned long long last_time = GetCPUCycles();
    unsigned int frame_count = 0;
    unsigned int current_fps = 0;

    // Szacujemy czêstotliwoœæ procesora w QEMU (np. oko³o 2-3 GHz)
    // 2 500 000 000 taktów to w przybli¿eniu jedna sekunda
    unsigned long long cycles_per_second = 2500000000ULL;
    unsigned long long cycle_accumulator = 0;

    // Ruchomy element, ¿ebyœmy widzieli, ¿e ekran ¿yje i ma p³ynnoœæ!
    unsigned int anim_x = 0;

    // G£ÓWNA PÊTLA RENDEROWANIA (Nasz Game Loop)
    while (1)
    {
        Video::FillScreen(0x0000FF);

        Video::DrawRect(100, 100, 100, 100, 0xFFFFFF);

        Video::Refresh();
    }
}