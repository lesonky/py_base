class EnumMixin():
  label_map = {}

  @classmethod
  def as_options(cls):
    return [{'label':x.label,'value':x.value} for x in cls]

  @classmethod
  def get_label_map(cls):
    return {[x.label]:x.value for x in cls}

  @property
  def label(self):
    return self.get_label_map().get(self.name,self.name)