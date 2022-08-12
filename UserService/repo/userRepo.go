package repo

import (
	"UserService/domain"
	"fmt"

	"gorm.io/gorm"
)

type UserRepo struct {
	db *gorm.DB
}

func ProvideUserRepo(db *gorm.DB) UserRepo {
	return UserRepo{
		db: db,
	}
}

func (repo *UserRepo) Create(user domain.User) (*domain.User, error) {
	res := repo.db.Create(&user)

	return &user, repo.HandleError(res)
}

func (repo *UserRepo) ReadAll() (*[]domain.User, error) {
	var users []domain.User
	res := repo.db.Find(&users)

	return &users, repo.HandleError(res)
}

func (repo *UserRepo) ReadById(id int64) (*domain.User, error) {
	var user *domain.User
	res := repo.db.Preload("User").Preload("User.Profile").Where("users.id = ?", id).First(user)

	return user, repo.HandleError(res)
}

func (repo *UserRepo) ReadByEmail(email string) (*domain.User, error) {
	var user domain.User
	res := repo.db.Preload("Roles").Where(&domain.User{Email: email}).First(&user)

	return &user, res.Error
}

func (r *UserRepo) HandleError(res *gorm.DB) error {
	if res.Error != nil && res.Error != gorm.ErrRecordNotFound {
		err := fmt.Errorf("error: %w", res.Error)
		return err
	}

	return nil
}
