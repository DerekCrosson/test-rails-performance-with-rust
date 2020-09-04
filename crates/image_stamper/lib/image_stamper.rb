require "helix_runtime"

begin
  require "image_stamper/native"
rescue LoadError
  warn "Unable to load image_stamper/native. Please run `rake build`"
end
