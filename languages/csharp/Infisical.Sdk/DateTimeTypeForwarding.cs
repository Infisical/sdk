#if NETSTANDARD2_0 || NETSTANDARD2_1
using System;
using System.Globalization;

namespace System
{
    public struct DateOnly
    {
        private readonly DateTime _date;
        
        public DateOnly(int year, int month, int day)
        {
            _date = new DateTime(year, month, day);
        }

        public static DateOnly Parse(string s)
        {
            return new DateOnly(DateTime.Parse(s).Ticks);
        }

        public string ToString(string format)
        {
            return _date.ToString(format);
        }

        public override string ToString()
        {
            return _date.ToString("yyyy-MM-dd");
        }

        public static implicit operator DateTime(DateOnly dateOnly) => dateOnly._date;
        public static explicit operator DateOnly(DateTime dateTime) => new DateOnly(dateTime.Year, dateTime.Month, dateTime.Day);

        // Additional constructor for ticks
        public DateOnly(long ticks)
        {
            _date = new DateTime(ticks);
        }
    }

    public struct TimeOnly
    {
        private readonly TimeSpan _timeSpan;
        
        public TimeOnly(int hour, int minute, int second = 0)
        {
            _timeSpan = new TimeSpan(hours: hour, minutes: minute, seconds: second);
        }

        public static TimeOnly Parse(string s)
        {
            var time = DateTime.Parse(s).TimeOfDay;
            return new TimeOnly(time.Hours, time.Minutes, time.Seconds);
        }

        public string ToString(string format)
        {
            return DateTime.Today.Add(_timeSpan).ToString(format);
        }

        public override string ToString()
        {
            return _timeSpan.ToString(@"hh\:mm\:ss");
        }

        public static implicit operator TimeSpan(TimeOnly timeOnly) => timeOnly._timeSpan;
        public static explicit operator TimeOnly(TimeSpan timeSpan) => new TimeOnly(timeSpan.Hours, timeSpan.Minutes, timeSpan.Seconds);
    }
}
#endif