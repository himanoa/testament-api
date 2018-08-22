-- This file should undo anything in `up.sql`
ALTER TABLE `entries` DROP COLUMN `user_id` INT NOT NULL;
