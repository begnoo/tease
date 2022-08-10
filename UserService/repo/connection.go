package repo

import (
	"UserService/domain"
	"log"

	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

// 0.0.0.0:49153
var db *gorm.DB = nil

func ProvideConnection() *gorm.DB {

	if db == nil {
		var err error
		db, err = gorm.Open(postgres.New(postgres.Config{
			DSN:                  "host=localhost user=postgres password=root dbname=userService port=5432 sslmode=disable",
			PreferSimpleProtocol: true,
		}), &gorm.Config{})

		if err != nil {
			panic("Failed to connect to users database")
		}

		migrate("users", &domain.User{})
		log.Printf("Successfuly connected to db...")
	}

	return db
}

func migrate(name string, domainStruct interface{}) {
	if db.AutoMigrate() != nil {
		log.Printf("Failed to inititialize users table...")
	} else {
		log.Printf("Initialized users table...")
	}
}
