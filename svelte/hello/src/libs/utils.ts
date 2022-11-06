import moment from 'moment'

export const formatDate = (dateString: string): string => {
  const date = moment(dateString, 'YYYYMMDDHHmmss')

  if (moment().isSame(date, 'day')) {
    return moment.duration(date.diff(moment(), 'milliseconds')).humanize(true)
  } else {
    return date.format('YYYY/MM/DD')
  }
}
