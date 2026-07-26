#include "kernel.hpp"
#include "../drivers/video.hpp"
#include "../include/limine.h"

__attribute__((used, section(".limine_requests_start")))
static volatile uint64_t limine_requests_start[] = LIMINE_REQUESTS_START_MARKER;

__attribute__((used, section(".limine_requests")))
static volatile struct limine_framebuffer_request framebuffer_request = {
    .id = LIMINE_FRAMEBUFFER_REQUEST_ID,
    .revision = 0
};

__attribute__((used, section(".limine_requests_end")))
static volatile uint64_t limine_requests_end[] = LIMINE_REQUESTS_END_MARKER;

// Pomocnicza funkcja czytaj¹ca takt procesora (RDTSC)
inline unsigned long long GetCPUCycles() {
    unsigned int low, high;
    __asm__ volatile ("rdtsc" : "=a" (low), "=d" (high));
    return ((unsigned long long)high << 32) | low;
}

extern "C" void KernelMain()
{
    auto response = framebuffer_request.response;

    if (response == nullptr)
    {
        while (1) asm("hlt");
    }

    auto framebuffer = response->framebuffers[0];

    unsigned int* address =
        (unsigned int*)framebuffer->address;

    unsigned int width =
        framebuffer->width;

    unsigned int height =
        framebuffer->height;

    unsigned int pitch =
        framebuffer->pitch;

    Video::Init(address, width, height, pitch);

    Video::FillScreen(0x00FF00FF);

    Video::DrawRect(0, 0, 40, 40, 0xFFFFFFFF);

    //Video::Refresh();
}
