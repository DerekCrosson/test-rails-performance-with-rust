class ImageController < ApplicationController
  def show
    initials = ('AA'..'ZZ').to_a.shuffle.first
    @filename = "#{initials}.png"
    ImageStamper.stamp(initials, @filename)
  end
end
