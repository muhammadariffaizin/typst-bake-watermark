#let image-processor = plugin("bake_watermark.wasm")

#let bake-img(
  img-path,
  watermark-text: "CONFIDENTIAL",
  opacity: 128,
  width: 100%,
) = {
  let raw-bytes = read(img-path, encoding: none)
  let baked = image-processor.bake_watermark(
    raw-bytes,
    bytes(watermark-text),
    bytes((calc.clamp(opacity, 0, 255),)),
  )
  image(baked, width: width)
}