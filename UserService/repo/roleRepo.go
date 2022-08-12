package repo

import (
	"UserService/domain"
	"fmt"

	"gorm.io/gorm"
)

type RoleRepo struct {
	db *gorm.DB
}

func ProvideRoleRepo(db *gorm.DB) RoleRepo {
	return RoleRepo{
		db: db,
	}
}

func (repo *RoleRepo) ReadByName(name string) (*domain.Role, error) {
	var role domain.Role
	res := repo.db.Where(&domain.Role{Name: name}).First(&role)

	return &role, res.Error
}

func (r *RoleRepo) HandleError(res *gorm.DB) error {
	if res.Error != nil && res.Error != gorm.ErrRecordNotFound {
		err := fmt.Errorf("error: %w", res.Error)
		return err
	}

	return nil
}
