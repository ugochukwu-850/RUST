//
#[derive(Debug, PartialEq, Copy, Clone)]
//shirtcolor num has two red and blue variantswhich should hold the colors the shirt can be in
pub enum ShirtColor {
    Red,
    Blue,
}

//a structure to hold the vector containing all the remaining shirst
pub struct Inventory {
    pub shirts: Vec<ShirtColor>,
}
//implementing the give away funtion and the most_stocked function
impl Inventory {
    /*Takes in a input of self ref and a user preference of type Option of  type enum of shirts
    returns the most stocked shirt if the Option is none and the shirt the user has input if it is
     */
    pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(||/*THE DOUBLE BRACKETS ARE CLOSURES */ self.most_stocked())
    }
    /*Returns a enumshirtcolor of the color of shirt that is most stocked
    Takes in a value of ref to self
     */
    pub fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}