CREATE TABLE pictures (
    id INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
    comment_id INT NOT NULL,
    path_to_picture TEXT NOT NULL,
    FOREIGN KEY (comment_id) REFERENCES comments(id) ON DELETE CASCADE
) ENGINE InnoDB;