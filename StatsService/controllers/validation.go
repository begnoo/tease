package controllers

import (
	"StatsService/errors"

	"github.com/go-playground/validator/v10"
)

var validate *validator.Validate = nil

func initValidator() *validator.Validate {
	if validate == nil {
		validate = validator.New()
		validate.RegisterValidation("source_name", sourceName)
	}

	return validate
}

func ValidateStruct(data interface{}) error {
	val := initValidator()
	err := val.Struct(data)

	return errors.NilOrError(err, &errors.ValidationError{Err: err})
}

var sourceName validator.Func = func(fl validator.FieldLevel) bool {
	sourceName := fl.Field().String()
	for _, char := range sourceName {
		if string(char) == " " {
			return false
		}
	}

	return true
}
