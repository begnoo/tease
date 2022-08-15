package request

type AddCollabaratorRequest struct {
	Collabarator string `validate:"required,email"`
	Source       int
}
