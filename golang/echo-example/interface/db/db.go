package db

type Db interface {
	First(interface{}, ...interface{}) interface{}
}
