package server

import (
	"UserService/domain"
	"UserService/request"

	"github.com/devfeel/mapper"
)

func InitMapper() {
	mapper.Register(&request.CreateUserRequest{})
	mapper.Register(&domain.User{})
}
