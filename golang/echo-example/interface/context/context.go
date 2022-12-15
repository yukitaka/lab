package context

type Context interface {
	JSON(int, interface{}) error
	Param(string) string
}
