package domain

import "gorm.io/gorm"

type Role struct {
	gorm.Model
	Name string
}

func (domain *Role) TableName() string {
	return "roles"
}
