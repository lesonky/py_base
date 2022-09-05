-- Add migration script here
CREATE TABLE IF NOT EXISTS role (
  `id` BIGINT NOT NULL AUTO_INCREMENT,
  `permissons` Json,
  PRIMARY KEY(`id`)
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4;
