# variable parameters
def report(*grades):
  total_grade = 0;
  for grade in grades:
    total_grade += grade
  return total_grade

# keywords parameters
def portrait(**kw):
  for k,v in kw.items():
    print(k,v)
    
if __name__ == '__main__':
  print('total grade is', report(10,30,20))
  portrait(age=24, country='China', education='bachelor')

#universal_func(*args, **kw)
