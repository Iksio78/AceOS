#include "kernel.hpp"
#include "../drivers/video.hpp"
#include "../include/limine.h"
#include "../include/limine.h"

__attribute__((used, section(".limine_requests")))
static volatile limine_framebuffer_request framebuffer_request = {
    .id = LIMINE_FRAMEBUFFER_REQUEST,
    .revision = 0
};

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
}
