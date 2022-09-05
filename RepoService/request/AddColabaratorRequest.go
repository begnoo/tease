package request

type AddCollabaratorRequest struct {
	Collabarator string `validate:"required,email" json:"collabarator"`
	Owner        string `validate:"required" json:"owner"`
	Name         string `validate:"required" json:"name"`
}
