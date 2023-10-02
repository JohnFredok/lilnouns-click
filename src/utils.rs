use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use regex::Regex;
use unidecode::unidecode;

pub fn truncate_and_clean_string(input: &str, limit: usize) -> String {
  // Check if the first line contains "#"
  let mut lines = input.lines();
  let first_line = lines.next().unwrap_or("");
  let mut remaining_text = if first_line.contains("#") {
    lines.collect::<Vec<&str>>().join("\n")
  } else {
    input.to_string()
  };

  // Remove HTML tags
  let tag_re = Regex::new(r"<[^>]*>").unwrap();
  remaining_text = tag_re.replace_all(&remaining_text, "").to_string();

  // Remove markdown tags
  let markdown_re = Regex::new(r"\*{1,2}|_{1,2}|#{1,6}|\`{1,3}|\[(.*?)\]\((.*?)\)").unwrap();
  remaining_text = markdown_re.replace_all(&remaining_text, "$1").to_string();

  // Remove URLs
  let url_re = Regex::new(r"https?://\S*").unwrap();
  remaining_text = url_re.replace_all(&remaining_text, "").to_string();

  // Replace new lines and double spaces with single space
  let new_line_re = Regex::new(r"\n+").unwrap();
  remaining_text = new_line_re.replace_all(&remaining_text, " ").to_string();
  let space_re = Regex::new(r" {2,}").unwrap();
  remaining_text = space_re.replace_all(&remaining_text, " ").to_string();

  // Remove all non-alphabet characters from the start of the string
  let start_re = Regex::new(r"^[^a-zA-Z]*").unwrap();
  remaining_text = start_re.replace(&remaining_text, "").to_string();

  // Transliterate non-ASCII characters to closest ASCII
  let remaining_text = unidecode(&remaining_text);

  // Remove non-ASCII characters
  let non_ascii_re = Regex::new(r"[^\x00-\x7F]").unwrap();
  let remaining_text = non_ascii_re.replace_all(&remaining_text, "").to_string();

  // Truncate and add ... if longer than limit
  if remaining_text.chars().count() > limit {
    let mut truncated = remaining_text.chars().take(limit).collect::<String>();
    truncated.push_str("...");
    truncated
  } else {
    remaining_text
  }
}

pub fn create_og_image(title: &str, description: &str) -> String {
  let non_alpha_numeric = Regex::new("[^a-zA-Z0-9 .]").unwrap();

  let title = non_alpha_numeric.replace_all(title, "");
  let description = non_alpha_numeric.replace_all(description, "");

  let cloudinary_id = "nekofar";
  let cloudinary_url = format!("https://res.cloudinary.com/{}/image/upload", cloudinary_id);

  let title_encoded = format!(
    "/l_text:{}_60_bold:{},co_rgb:000000,c_fit,w_1000,h_200",
    utf8_percent_encode("Londrina Solid", NON_ALPHANUMERIC),
    utf8_percent_encode(&title, NON_ALPHANUMERIC)
  );

  let description_encoded = format!(
    "/l_text:{}_40:{},co_rgb:00000080,c_fit,w_1000",
    utf8_percent_encode("Londrina Solid", NON_ALPHANUMERIC),
    utf8_percent_encode(&description, NON_ALPHANUMERIC)
  );

  let parts = vec![
    &cloudinary_url,
    "/b_rgb:D4D7E1",
    "/c_scale,h_630,w_1200",
    "/l_black_noggle/c_scale,w_300/e_screen,fl_layer_apply,g_north",
    &title_encoded,
    "/fl_layer_apply,g_south_west,x_100,y_230",
    &description_encoded,
    "/fl_layer_apply,g_south_west,x_100,y_70",
    "/f_auto,q_auto:eco",
    "/blank.png",
  ];

  parts.join("")
}
