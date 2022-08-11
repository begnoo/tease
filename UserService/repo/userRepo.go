package repo

import (
	"UserService/domain"
	"UserService/errors"
	"fmt"
	"log"

	"gorm.io/gorm"
)

type UserRepo struct {
	logger *log.Logger
	db     *gorm.DB
}

func ProvideUserRepo(logger *log.Logger, db *gorm.DB) UserRepo {
	return UserRepo{
		logger: logger,
		db:     db,
	}
}

func (repo *UserRepo) Create(user domain.User) (*domain.User, error) {
	// TODO: handle ovde ako ima error posle
	same_email, _ := repo.ReadByEmail(user.Email)

	if same_email != nil {
		return nil, &errors.SameEmailError{Message: fmt.Sprintf("{'error': 'Email '%s' already taken'}", user.Email)}
	}

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
	res := repo.db.Where(&domain.User{Email: email}).First(&user)

	return &user, repo.HandleError(res)
}

func (r *UserRepo) HandleError(res *gorm.DB) error {
	if res.Error != nil && res.Error != gorm.ErrRecordNotFound {
		err := fmt.Errorf("error: %w", res.Error)
		r.logger.Fatal(err)
		return err
	}

	return nil
}
