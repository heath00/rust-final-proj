initSidebarItems({"fn":[["decrypt","Decrypts the given image with the given swirl in order to restore the orignial image saves to the given directory Filename: &str, export_directory: &str, degrees u32 -> puts image into directory as decrImg.jpg"],["decrypt_many","Takes a vector of angles to try and reverse many encryptions to go back to the original saves it to the given directory. Panics if the given vector of degrees is empty. Filename: &str, export_directory: &str, degrees Vec -> puts image into directory as decrImg.jpg"],["encrypt","Encrypts the given image with the given angle of swirl in order to disguise image saves the image in the given directory Filename: &str, export_directory: &str, degrees u32 -> puts image into directory as encrImg.jpg"],["encrypt_many","Takes a vector of angles to do many encryptions on the image saves to the given directory. As you add more numbers the image will be harder to recognize when decrypted. Panics if the given vector of degrees is empty. Filename: &str, export_directory: &str, degrees Vec -> puts image into given directory as encrImg.jpg"]]});