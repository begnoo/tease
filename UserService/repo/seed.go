package repo

import "UserService/domain"

func seed() {
	seed_roles()
}

func seed_roles() {
	var count int64
	db := ProvideConnection()
	db.Find(&domain.Role{}).Count(&count)
	if count == 0 {
		db.Create(&domain.Role{Name: "ROLE_ADMIN"})
		db.Create(&domain.Role{Name: "ROLE_USER"})
	}
}
