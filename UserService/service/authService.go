package service

import (
	"UserService/errors"
	"UserService/repo"
	"UserService/security"

	"github.com/golang-jwt/jwt/v4"
)

type AuthService struct {
	userRepo *repo.UserRepo
	roleRepo *repo.RoleRepo
}

func ProvideAuthService(userRepo repo.UserRepo, roleRepo repo.RoleRepo) AuthService {
	return AuthService{
		userRepo: &userRepo,
		roleRepo: &roleRepo,
	}
}

func (service *AuthService) Login(creds security.Credidentials) (string, error) {
	user, err := service.userRepo.ReadByEmail(creds.Email)

	if err != nil {
		return "", err
	}

	if !security.CheckPasswordHash(creds.Password, user.Password) {
		return "", &errors.FailedAuthorization{Message: "Wrong password!"}
	}

	token, err := security.GenerateJWT(creds.Email, user.Roles[0].Name, false)

	return token, err
}

func (service *AuthService) GenerateBackendJwt(token *jwt.Token) (string, error) {

	claims_map := token.Claims.(jwt.MapClaims)
	new_token, err := security.GenerateJWT(claims_map["email"].(string), claims_map["role"].(string), true)

	return new_token, err
}
