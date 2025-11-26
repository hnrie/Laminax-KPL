from PIL import Image, ImageDraw, ImageFont, ImageFilter, ImageEnhance, ImageOps, ImageChops
import os

def image_load(path):
    try:
        return Image.open(path)
    except Exception as e:
        raise IOError(f"Cannot load image: {e}")

def image_save(img, path, format=None):
    try:
        if format:
            img.save(path, format=format)
        else:
            img.save(path)
        return None
    except Exception as e:
        raise IOError(f"Cannot save image: {e}")

def image_new(mode, size, color=0):
    if isinstance(size, list):
        size = tuple(size)
    if isinstance(color, list):
        color = tuple(color)
    return Image.new(mode, size, color)

def image_size(img):
    return list(img.size)

def image_mode(img):
    return img.mode

def image_format(img):
    return img.format if img.format else "unknown"

def image_resize(img, size, resample=None):
    if isinstance(size, list):
        size = tuple(size)
    if resample is None:
        return img.resize(size, Image.LANCZOS)
    return img.resize(size, resample)

def image_crop(img, box):
    if isinstance(box, list):
        box = tuple(box)
    return img.crop(box)

def image_rotate(img, angle, expand=False, fillcolor=None):
    if isinstance(fillcolor, list):
        fillcolor = tuple(fillcolor)
    return img.rotate(angle, expand=expand, fillcolor=fillcolor)

def image_flip_horizontal(img):
    return img.transpose(Image.FLIP_LEFT_RIGHT)

def image_flip_vertical(img):
    return img.transpose(Image.FLIP_TOP_BOTTOM)

def image_transpose(img):
    return img.transpose(Image.TRANSPOSE)

def image_paste(img, im, box=None, mask=None):
    if isinstance(box, list):
        box = tuple(box)
    img.paste(im, box, mask)
    return None

def image_copy(img):
    return img.copy()

def image_convert(img, mode):
    return img.convert(mode)

def image_thumbnail(img, size):
    if isinstance(size, list):
        size = tuple(size)
    img.thumbnail(size, Image.LANCZOS)
    return None

def image_blur(img, radius=2):
    return img.filter(ImageFilter.GaussianBlur(radius))

def image_sharpen(img):
    return img.filter(ImageFilter.SHARPEN)

def image_smooth(img):
    return img.filter(ImageFilter.SMOOTH)

def image_smooth_more(img):
    return img.filter(ImageFilter.SMOOTH_MORE)

def image_edge_enhance(img):
    return img.filter(ImageFilter.EDGE_ENHANCE)

def image_edge_enhance_more(img):
    return img.filter(ImageFilter.EDGE_ENHANCE_MORE)

def image_find_edges(img):
    return img.filter(ImageFilter.FIND_EDGES)

def image_contour(img):
    return img.filter(ImageFilter.CONTOUR)

def image_emboss(img):
    return img.filter(ImageFilter.EMBOSS)

def image_detail(img):
    return img.filter(ImageFilter.DETAIL)

def image_box_blur(img, radius=2):
    return img.filter(ImageFilter.BoxBlur(radius))

def image_unsharp_mask(img, radius=2, percent=150, threshold=3):
    return img.filter(ImageFilter.UnsharpMask(radius, percent, threshold))

def image_brightness(img, factor):
    enhancer = ImageEnhance.Brightness(img)
    return enhancer.enhance(factor)

def image_contrast(img, factor):
    enhancer = ImageEnhance.Contrast(img)
    return enhancer.enhance(factor)

def image_color(img, factor):
    enhancer = ImageEnhance.Color(img)
    return enhancer.enhance(factor)

def image_sharpness(img, factor):
    enhancer = ImageEnhance.Sharpness(img)
    return enhancer.enhance(factor)

def image_grayscale(img):
    return ImageOps.grayscale(img)

def image_invert(img):
    return ImageOps.invert(img)

def image_equalize(img):
    return ImageOps.equalize(img)

def image_autocontrast(img):
    return ImageOps.autocontrast(img)

def image_posterize(img, bits):
    return ImageOps.posterize(img, int(bits))

def image_solarize(img, threshold=128):
    return ImageOps.solarize(img, int(threshold))

def image_mirror(img):
    return ImageOps.mirror(img)

def image_fit(img, size, method=None):
    if isinstance(size, list):
        size = tuple(size)
    if method is None:
        return ImageOps.fit(img, size, Image.LANCZOS)
    return ImageOps.fit(img, size, method)

def image_expand(img, border=0, fill=0):
    if isinstance(fill, list):
        fill = tuple(fill)
    return ImageOps.expand(img, border, fill)

def image_pad(img, size, color=None):
    if isinstance(size, list):
        size = tuple(size)
    if isinstance(color, list):
        color = tuple(color)
    return ImageOps.pad(img, size, color=color)

def image_add(img1, img2, scale=1.0, offset=0):
    return ImageChops.add(img1, img2, scale, offset)

def image_subtract(img1, img2, scale=1.0, offset=0):
    return ImageChops.subtract(img1, img2, scale, offset)

def image_multiply(img1, img2):
    return ImageChops.multiply(img1, img2)

def image_screen(img1, img2):
    return ImageChops.screen(img1, img2)

def image_difference(img1, img2):
    return ImageChops.difference(img1, img2)

def image_darker(img1, img2):
    return ImageChops.darker(img1, img2)

def image_lighter(img1, img2):
    return ImageChops.lighter(img1, img2)

def image_blend(img1, img2, alpha):
    return Image.blend(img1, img2, alpha)

def image_composite(img1, img2, mask):
    return Image.composite(img1, img2, mask)

def image_draw(img):
    return ImageDraw.Draw(img)

def draw_line(draw, xy, fill=None, width=0):
    if isinstance(xy, list):
        xy = [tuple(p) if isinstance(p, list) else p for p in xy]
    if isinstance(fill, list):
        fill = tuple(fill)
    draw.line(xy, fill=fill, width=width)
    return None

def draw_rectangle(draw, xy, fill=None, outline=None, width=1):
    if isinstance(xy, list):
        xy = tuple(xy)
    if isinstance(fill, list):
        fill = tuple(fill)
    if isinstance(outline, list):
        outline = tuple(outline)
    draw.rectangle(xy, fill=fill, outline=outline, width=width)
    return None

def draw_ellipse(draw, xy, fill=None, outline=None, width=1):
    if isinstance(xy, list):
        xy = tuple(xy)
    if isinstance(fill, list):
        fill = tuple(fill)
    if isinstance(outline, list):
        outline = tuple(outline)
    draw.ellipse(xy, fill=fill, outline=outline, width=width)
    return None

def draw_circle(draw, center, radius, fill=None, outline=None, width=1):
    if isinstance(center, list):
        center = tuple(center)
    if isinstance(fill, list):
        fill = tuple(fill)
    if isinstance(outline, list):
        outline = tuple(outline)
    x, y = center
    xy = [x - radius, y - radius, x + radius, y + radius]
    draw.ellipse(xy, fill=fill, outline=outline, width=width)
    return None

def draw_polygon(draw, xy, fill=None, outline=None, width=1):
    if isinstance(xy, list):
        xy = [tuple(p) if isinstance(p, list) else p for p in xy]
    if isinstance(fill, list):
        fill = tuple(fill)
    if isinstance(outline, list):
        outline = tuple(outline)
    draw.polygon(xy, fill=fill, outline=outline, width=width)
    return None

def draw_arc(draw, xy, start, end, fill=None, width=1):
    if isinstance(xy, list):
        xy = tuple(xy)
    if isinstance(fill, list):
        fill = tuple(fill)
    draw.arc(xy, start, end, fill=fill, width=width)
    return None

def draw_chord(draw, xy, start, end, fill=None, outline=None, width=1):
    if isinstance(xy, list):
        xy = tuple(xy)
    draw.chord(xy, start, end, fill=fill, outline=outline, width=width)
    return None

def draw_pieslice(draw, xy, start, end, fill=None, outline=None, width=1):
    if isinstance(xy, list):
        xy = tuple(xy)
    draw.pieslice(xy, start, end, fill=fill, outline=outline, width=width)
    return None

def draw_text(draw, xy, text, fill=None, font=None):
    if isinstance(xy, list):
        xy = tuple(xy)
    if isinstance(fill, list):
        fill = tuple(fill)
    draw.text(xy, text, fill=fill, font=font)
    return None

def draw_point(draw, xy, fill=None):
    if isinstance(xy, list):
        xy = tuple(xy)
    if isinstance(fill, list):
        fill = tuple(fill)
    draw.point(xy, fill=fill)
    return None

def image_get_pixel(img, xy):
    if isinstance(xy, list):
        xy = tuple(xy)
    pixel = img.getpixel(xy)
    if isinstance(pixel, tuple):
        return list(pixel)
    return pixel

def image_put_pixel(img, xy, value):
    if isinstance(xy, list):
        xy = tuple(xy)
    if isinstance(value, list):
        value = tuple(value)
    img.putpixel(xy, value)
    return None

def image_get_bands(img):
    return list(img.getbands())

def image_split(img):
    return list(img.split())

def image_merge(mode, bands):
    if isinstance(bands, list):
        bands = tuple(bands)
    return Image.merge(mode, bands)

def image_alpha_composite(img1, img2):
    return Image.alpha_composite(img1, img2)

def image_show(img):
    img.show()
    return None

IMAGE_FUNCTIONS = {
    'image_load': image_load,
    'image_save': image_save,
    'image_new': image_new,
    'image_size': image_size,
    'image_mode': image_mode,
    'image_format': image_format,
    'image_resize': image_resize,
    'image_crop': image_crop,
    'image_rotate': image_rotate,
    'image_flip_horizontal': image_flip_horizontal,
    'image_flip_vertical': image_flip_vertical,
    'image_transpose': image_transpose,
    'image_paste': image_paste,
    'image_copy': image_copy,
    'image_convert': image_convert,
    'image_thumbnail': image_thumbnail,
    'image_blur': image_blur,
    'image_sharpen': image_sharpen,
    'image_smooth': image_smooth,
    'image_smooth_more': image_smooth_more,
    'image_edge_enhance': image_edge_enhance,
    'image_edge_enhance_more': image_edge_enhance_more,
    'image_find_edges': image_find_edges,
    'image_contour': image_contour,
    'image_emboss': image_emboss,
    'image_detail': image_detail,
    'image_box_blur': image_box_blur,
    'image_unsharp_mask': image_unsharp_mask,
    'image_brightness': image_brightness,
    'image_contrast': image_contrast,
    'image_color': image_color,
    'image_sharpness': image_sharpness,
    'image_grayscale': image_grayscale,
    'image_invert': image_invert,
    'image_equalize': image_equalize,
    'image_autocontrast': image_autocontrast,
    'image_posterize': image_posterize,
    'image_solarize': image_solarize,
    'image_mirror': image_mirror,
    'image_fit': image_fit,
    'image_expand': image_expand,
    'image_pad': image_pad,
    'image_add': image_add,
    'image_subtract': image_subtract,
    'image_multiply': image_multiply,
    'image_screen': image_screen,
    'image_difference': image_difference,
    'image_darker': image_darker,
    'image_lighter': image_lighter,
    'image_blend': image_blend,
    'image_composite': image_composite,
    'image_draw': image_draw,
    'draw_line': draw_line,
    'draw_rectangle': draw_rectangle,
    'draw_ellipse': draw_ellipse,
    'draw_circle': draw_circle,
    'draw_polygon': draw_polygon,
    'draw_arc': draw_arc,
    'draw_chord': draw_chord,
    'draw_pieslice': draw_pieslice,
    'draw_text': draw_text,
    'draw_point': draw_point,
    'image_get_pixel': image_get_pixel,
    'image_put_pixel': image_put_pixel,
    'image_get_bands': image_get_bands,
    'image_split': image_split,
    'image_merge': image_merge,
    'image_alpha_composite': image_alpha_composite,
    'image_show': image_show,
}
