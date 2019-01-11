CREATE TABLE forums (
    id INT PRIMARY KEY AUTO_INCREMENT, 
    name TEXT NOT NULL, 
    category_id INT NOT NULL,
    max_threads_number INT NOT NULL,
    cur_threads_number INT NOT NULL,
    FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE RESTRICT
) ENGINE InnoDB;