CREATE DATABASE `outing_development` DEFAULT CHARACTER SET utf8mb4;

USE `outing_development`;

CREATE TABLE `experiences` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `title` varchar(255) DEFAULT NULL,
  `content` mediumtext,
  `experiencable_id` int(11) DEFAULT NULL,
  `parent_id` int(11) DEFAULT NULL,
  `favorites_count` int(11) DEFAULT '0',
  `enjoy_time_id` int(11) DEFAULT NULL,
  `private` tinyint(1) NOT NULL DEFAULT '0',
  `created_at` datetime DEFAULT NULL,
  `updated_at` datetime DEFAULT NULL,
  `experiencable_type` varchar(40) DEFAULT NULL,
  `outing_on` date DEFAULT NULL,
  `elementary_rating` int(11) DEFAULT NULL,
  `toddler_rating` int(11) DEFAULT NULL,
  `adult_price_id` int(11) DEFAULT NULL,
  `elementary_price_id` int(11) DEFAULT NULL,
  `toddler_price_id` int(11) DEFAULT NULL,
  `parent_name` varchar(255) DEFAULT NULL,
  `publish` tinyint(1) NOT NULL DEFAULT '1',
  `upload_email` varchar(40) DEFAULT NULL,
  `upload_mailbox` varchar(255) DEFAULT NULL,
  `thanks_count` int(11) NOT NULL DEFAULT '0',
  `first_published_at` datetime DEFAULT NULL,
  `facility_genre_id` int(11) DEFAULT NULL,
  `companion_id` int(11) DEFAULT NULL,
  `created_by` int(11) NOT NULL DEFAULT '0',
  PRIMARY KEY (`id`),
  UNIQUE KEY `index_experiences_on_upload_email` (`upload_email`),
  KEY `index_experiences_on_parent_id` (`parent_id`),
  KEY `index_experiences_on_experiencable_id_and_experiencable_type` (`experiencable_id`,`experiencable_type`),
  KEY `index_experiences_on_facility_genre_id` (`facility_genre_id`),
  KEY `index_experiences_on_companion_id` (`companion_id`)
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4;

CREATE TABLE `favorites` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `favorable_type` varchar(30) NOT NULL,
  `favorable_id` int(11) NOT NULL,
  `favorable_parent_id` int(11) DEFAULT NULL,
  `parent_id` int(11) DEFAULT NULL,
  `created_at` datetime DEFAULT NULL,
  `updated_at` datetime DEFAULT NULL,
  `user_cookie_value` varchar(40) DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `index_favorites_on_parent_id_and_favorable_id_and_favorable_type` (`parent_id`,`favorable_id`,`favorable_type`),
  UNIQUE KEY `index_favorites_on_uniq_cookie_and_ids` (`user_cookie_value`,`favorable_id`,`favorable_type`),
  KEY `index_favorites_on_favorable_id_and_favorable_type` (`favorable_id`,`favorable_type`),
  KEY `index_favorites_on_favorable_parent_id` (`favorable_parent_id`),
  KEY `index_favorites_on_parent_id` (`parent_id`),
  KEY `index_favorites_on_created_favorable` (`created_at`,`favorable_id`,`favorable_type`)
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4;

CREATE TABLE `thanks` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `parent_id` int(11) DEFAULT NULL,
  `experience_id` int(11) DEFAULT NULL,
  `created_at` datetime DEFAULT NULL,
  `updated_at` datetime DEFAULT NULL,
  `user_cookie_value` varchar(255) COLLATE utf8_unicode_ci DEFAULT NULL,
  `thankable_id` int(11) DEFAULT NULL,
  `thankable_type` varchar(40) COLLATE utf8_unicode_ci DEFAULT NULL,
  `received_parent_id` int(11) DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `index_thanks_on_parent_id` (`parent_id`),
  KEY `index_thanks_on_experience_id` (`experience_id`),
  KEY `index_thanks_on_thankable_id_and_thankable_type` (`thankable_id`,`thankable_type`),
  KEY `index_thanks_on_received_parent_id` (`received_parent_id`)
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;

CREATE TABLE `users` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `login` varchar(40) DEFAULT NULL,
  `email` varchar(100) DEFAULT NULL,
  `crypted_password` varchar(40) DEFAULT NULL,
  `salt` varchar(40) DEFAULT NULL,
  `remember_token` varchar(40) DEFAULT NULL,
  `remember_token_expires_at` datetime DEFAULT NULL,
  `type` varchar(40) DEFAULT NULL,
  `created_at` datetime DEFAULT NULL,
  `updated_at` datetime DEFAULT NULL,
  `name` varchar(40) DEFAULT NULL,
  `rep_first_name` varchar(20) DEFAULT NULL,
  `rep_family_name` varchar(20) DEFAULT NULL,
  `phone_number` varchar(15) DEFAULT NULL,
  `prefecture_id` int(11) DEFAULT NULL,
  `address` mediumtext,
  `activation_code` varchar(40) DEFAULT NULL,
  `activated_at` datetime DEFAULT NULL,
  `failed_attempts` int(11) NOT NULL DEFAULT '0',
  `locked_at` datetime DEFAULT NULL,
  `rep_first_name_kana` varchar(20) DEFAULT NULL,
  `rep_family_name_kana` varchar(20) DEFAULT NULL,
  `role_cd` int(11) DEFAULT '0',
  `password_too_short` tinyint(1) DEFAULT '0',
  `post` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `index_users_on_login` (`login`),
  UNIQUE KEY `index_users_on_email` (`email`),
  KEY `index_users_on_type` (`type`),
  KEY `index_users_on_remember_token` (`remember_token`)
) ENGINE=InnoDB AUTO_INCREMENT=103181 DEFAULT CHARSET=utf8mb4;
