package domain

import "gorm.io/gorm"

type User struct {
	gorm.Model
	Username string
	Email    string `mapper:"Email"`
	Password string

	Profile Profile
	Roles   []Role `gorm:"many2many:users_roles;"`
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
