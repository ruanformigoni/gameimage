///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : image
///

#pragma once

#include <filesystem>
#include <boost/gil.hpp>
#include <boost/gil/extension/io/jpeg.hpp>
#include <boost/gil/extension/io/png.hpp>
#include <boost/gil/extension/numeric/sampler.hpp>
#include <boost/gil/extension/numeric/resample.hpp>

#include "log.hpp"
#include "../enum.hpp"
#include "subprocess.hpp"

namespace
{

namespace fs = std::filesystem;
namespace gil = boost::gil;

// format() {{{
ns_enum::ImageFormat format(fs::path const& path_file_src)
{
  // File extension
  std::string ext = path_file_src.extension();

  // // Check result
  "Empty file extension"_throw_if([&]{ return ext.empty(); });

  // // Remove the leading dot
  ext.erase(ext.begin());

  // Get enum option
  ns_enum::ImageFormat image_format;

  // Check image type
  "Image type '{}' is not supported, supported types are '.jpg, .jpeg, .png'"_try(
    [&]{ image_format = ns_enum::from_string<ns_enum::ImageFormat>(ext); }
    , ext
  );

  return image_format;
} // format() }}}

} // namespace

namespace ns_image
{

// resize() {{{
inline void resize(fs::path const& path_file_src 
  , fs::path const& path_file_dst
  , int64_t width
  , int64_t height)
{
  ns_log::write('i', "Reading image from ", path_file_src);
  switch ( format(path_file_src) )
  {
    // Convert jpg to png
    case ns_enum::ImageFormat::JPG:
    case ns_enum::ImageFormat::JPEG:
      ns_log::write('i', "Reading as 'jpg'");
      break;
    // Copy
    case ns_enum::ImageFormat::PNG:
      ns_log::write('i', "Reading as 'png'");
      break;
  } // switch

  // Resize
  // Boost gil generates bad results when resizing, use magick for now
  fs::path path_file_resized = path_file_dst.parent_path() / "cropped.png";
  auto optional_path_file_magick = ns_subprocess::search_path("magick");
  ereturn_if(not optional_path_file_magick, "Could not find magick binary");
  (void) ns_subprocess::Subprocess(*optional_path_file_magick)
    .with_piped_outputs()
    .with_args(path_file_src, "-resize", "{}x{}"_fmt(width, height), path_file_resized)
    .spawn()
    .wait();

  // Crop
  gil::rgba8_image_t img;
  gil::read_and_convert_image(path_file_resized, img, gil::png_tag());

  ns_log::write('i', "Image width and height ", img.width(), "x", img.height());
  ns_log::write('i', "Target width and height ", width, "x", height);

  // Get greatest difference between dimmensions
  int difference = std::min(width > img.width()? img.width() - width : 0
    , (height > img.height())? img.height() - height : 0);

  ns_log::write('i', "Difference: ", difference);

  // Remove difference from both dimmensions (proportionally the same, but fits the image)
  width  += difference;
  height += difference;

  ns_log::write('i', "Adjusted width and height to ", width, "x", height);

  // Calculate desired and current aspected ratios
  double src_aspect = static_cast<double>(img.width()) / img.height();
  double dst_aspect = static_cast<double>(width) / height;

  // Calculate novel dimensions that preserve the aspect ratio
  int width_new  = (src_aspect >  dst_aspect)? static_cast<int>(src_aspect * height) : width;
  int height_new = (src_aspect <= dst_aspect)? static_cast<int>(width / src_aspect ) : height;

  // Calculate crop
  int crop_x = (width_new - width) / 2;
  int crop_y = (height_new - height) / 2;

  // Crop the image
  auto view_img_cropped = gil::subimage_view(gil::view(img), crop_x, crop_y, width, height);

  // Save cropped image
  ns_log::write('i', "Writing image to ", path_file_dst);
  gil::write_view(path_file_dst, view_img_cropped, gil::png_tag());
} // resize() }}}

// grayscale() {{{
inline void grayscale(fs::path const& path_file_src, fs::path const& path_file_dst)
{
  ns_log::write('i', "Reading image from ", path_file_src);
  gil::rgba8_image_t img;
  switch ( format(path_file_src) )
  {
    // Convert jpg to png
    case ns_enum::ImageFormat::JPG:
    case ns_enum::ImageFormat::JPEG:
      ns_log::write('i', "Reading as 'jpg'");
      gil::read_and_convert_image(path_file_src, img, gil::jpeg_tag());
      break;
    // Copy
    case ns_enum::ImageFormat::PNG:
      ns_log::write('i', "Reading as 'png'");
      gil::read_and_convert_image(path_file_src, img, gil::png_tag());
      break;
  } // switch

  ns_log::write('i', "Writing grayscale image to ", path_file_dst);
  gil::rgba8_image_t img_gray(img.dimensions());
  auto view_const = gil::const_view(img);
  auto view_gray = gil::view(img_gray);
  for (int y = 0; y < view_const.height(); ++y)
  {
    for (int x = 0; x < view_const.width(); ++x)
    {
      auto pixel = view_const(x, y);
      auto gray_val = static_cast<uint8_t>(0.299 * pixel[0] + 0.587 * pixel[1] + 0.114 * pixel[2]);
      auto alpha_val = pixel[3];
      view_gray(x,y) = gil::rgba8_pixel_t(gray_val, gray_val, gray_val, alpha_val);
    } // for
  } // for
  gil::write_view(path_file_dst, view_gray, gil::png_tag());
  
} // grayscale() }}}

} // namespace ns_image

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
