#let image-processor = plugin("bake_watermark.wasm")

#let secure-image(img-path, watermark-text: "CONFIDENTIAL", opacity: 128, width: 100%) = {
  let raw-bytes = read(img-path, encoding: none)
  let baked = image-processor.bake_watermark(
    raw-bytes,
    bytes(watermark-text),
    bytes((calc.clamp(opacity, 0, 255),)),
  )
  image(baked, width: width)
}

= Secure Document
This image secured with a watermark embedded directly into the image data.
#secure-image("input.png", watermark-text: "THIS IS A WATERMARK", opacity: 100)