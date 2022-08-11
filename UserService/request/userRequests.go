package request

type CreateUserRequest struct {
	Email     string `validate:"required,email"`
	Username  string `validate:"required,min=4,max=10"`
	Password  string `validate:"required,min=8"`
	FirstName string `validate:"required"`
	LastName  string `validate:"required"`
}

func NewCreateUserRequest() *CreateUserRequest {
	return &CreateUserRequest{}
}
