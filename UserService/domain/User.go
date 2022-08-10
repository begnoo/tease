package domain

import "gorm.io/gorm"

type User struct {
	gorm.Model
	Username string
	Email    string
	Password string

	Profile Profile
}

func (domain *User) TableName() string {
	return "users"
}

type Profile struct {
	gorm.Model
	FirstName string
	LastName  string

	UserID uint
}
