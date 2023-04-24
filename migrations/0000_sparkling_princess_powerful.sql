CREATE TABLE `Post` (
	`slug` varchar(256) PRIMARY KEY NOT NULL,
	`title` varchar(256) NOT NULL,
	`content` text,
	`published` boolean NOT NULL DEFAULT false,
	`created_at` timestamp(2) NOT NULL DEFAULT now(2)
);
--> statement-breakpoint
CREATE TABLE `Tag` (
	`name` varchar(256) PRIMARY KEY NOT NULL
);
--> statement-breakpoint
CREATE TABLE `TagsOnPosts` (
	`post_slug` varchar(256) NOT NULL,
	`tag_name` varchar(256) NOT NULL
);
--> statement-breakpoint
ALTER TABLE `TagsOnPosts` ADD PRIMARY KEY(`post_slug`,`tag_name`);
--> statement-breakpoint
CREATE INDEX `created_at_idx` ON `Post` (`created_at`);