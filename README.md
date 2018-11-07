# Rustygl - Learn openGL and Rust by doing stuff

## Motivation

Games were my doorway into the world of computers and I have always wanted to program games. C was too low level and error prone, and the other HLL weren't as powerful as C. Rust is at the moment, IMHO, the perfect balance between being as powerful as C with a lot of niceties that will make programs a lot safer and I find it very fun and productive. 

I actually started off with Vulkan, but very early in the process, realized that I do not know the modern graphics pipeline at all and took my ambitions a notch lower, to understand how the pipeline works and then once sufficiently good at it, understand why Vulkan and then use Vulkan to write graphics applications. 

This project is basically my attempt at learning openGL and sharpening my Rust skills and knowledge along with way


## Running the code

`cargo build && cargo run` should ideall do the trick. I started writing this on a Macbook and haven't had an issue linking it against OpenGL library/drivers. Your mileage may vary. 

### Caveat
I am running into issue where sdl2 with the `bundled` and `static-link` features enabled in `Cargo.toml` doesn't work in Linux. When `bundled` is enabled, sdl2 downloads and compiles sdl2 from source and it seems there is some cmake setting that I might be missing.  

To circumvent this, I manually installed `libsdl2-dev` on my Linux machine and removed the 2 above mentioned features from my Cargo.toml and it works fine. Any help debugging/reproducing this issue would be nice (a solution would be very much appreciated :))
