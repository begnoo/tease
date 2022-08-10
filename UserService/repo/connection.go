package repo

import (
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

// 0.0.0.0:49153
var db *gorm.DB = nil

func GetConnection() *gorm.DB {

	if db == nil {
		var err error
		db, err = gorm.Open(postgres.New(postgres.Config{
			DSN:                  "host=0.0.0.0 user=postgres password=postgres dbname=users port=49153 sslmode=disable",
			PreferSimpleProtocol: true,
		}), &gorm.Config{})

		if err != nil {
			panic("Failed to connect to users database")
		}
	}

	return db
}
