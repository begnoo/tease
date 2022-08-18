package controllers

import (
	"UserService/errors"

	"github.com/go-playground/validator/v10"
)

var validate *validator.Validate = nil

func initValidator() *validator.Validate {
	if validate == nil {
		validate = validator.New()
	}

	return validate
}

func ValidateStruct(data interface{}) error {
	val := initValidator()
	err := val.Struct(data)

	return errors.NilOrError(err, &errors.RepoError{Err: err})
}
