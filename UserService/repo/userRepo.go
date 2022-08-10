package repo

import (
	"UserService/domain"
	"fmt"
	"log"
	"os"

	"gorm.io/gorm"
)

type UserRepo struct {
	logger *log.Logger
	db     *gorm.DB
}

func NewUserRepo() UserRepo {
	return UserRepo{
		logger: log.New(os.Stdout, "USER_REPO", 1),
		db:     GetConnection(),
	}
}

func (repo *UserRepo) Create(user domain.User) (domain.User, error) {
	res := repo.db.Create(user)

	return user, repo.HandleError(res)
}

func (repo *UserRepo) ReadAll() (*[]domain.User, error) {
	var users *[]domain.User
	res := repo.db.Take(users, "User")

	return users, repo.HandleError(res)
}

func (repo *UserRepo) ReadById(id int64) (domain.User, error) {
	var user domain.User
	res := repo.db.Preload("User").Where("users.id = ?", id).First(user)

	return user, repo.HandleError(res)
}

func (repo *UserRepo) ReadByEmail(email string) (domain.User, error) {
	var user domain.User
	res := repo.db.Preload("User").Where("users.email = ?", email).First(user)

	return user, repo.HandleError(res)
}

func (r *UserRepo) HandleError(res *gorm.DB) error {
	if res.Error != nil && res.Error != gorm.ErrRecordNotFound {
		err := fmt.Errorf("error: %w", res.Error)
		r.logger.Fatal(err)
		return err
	}

	return nil
}
