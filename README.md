# nanoimage

Fork of an [awesome library](https://github.com/des256/image_formats/tree/master) by Desmond Germans, 2019
With some code from Makepad  
And some hacks to make it easier to use with *quads.

https://github.com/des256/image_formats/tree/master
https://github.com/makepad/makepad/tree/master/draw/image_formats

Why not image? Clean compilation time for image: 44s and +600Mb in /target, for image_formats: 0.5s and +6Mb in /target

Why not `zune` (https://github.com/etemesi254/zune-image.git)? Honestly, I just noticed `zune` existance too late. `zune` is a lot better.

## Intended use

`nanoimage` is a really bad choice for arbitary user-inputed images. It is slow, it has bugs. 

If the choice is between `showcase_my_library(include_bytes!("rgba_array.bytes"))` and `showcase_my_library(nanoimage::decode(include_bytes!("one_fixed_example_image.png")))` - `nanoimage` may be a reasonable choice.
