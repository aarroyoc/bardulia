extends Spatial


# Declare member variables here. Examples:
# var a = 2
# var b = "text"


# Called when the node enters the scene tree for the first time.
func _ready():
	var animation = self.get_node("AnimationPlayer")
	animation.play("Idle")


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	var animation = self.get_node("AnimationPlayer")
	if Input.is_action_pressed("move_forward"):
		if not animation.current_animation == "Run":
			animation.play("Run")
		var rot_x = self.rotation.x
		var rot_z = self.rotation.z
		self.translate(Vector3(3.0*delta*sin(rot_x), 0.0, 3.0*delta*cos(rot_z)))
	else:
		animation.play("Idle")
	if Input.is_action_pressed("rotate_left"):
		self.rotate_y(1.0*delta)
	if Input.is_action_pressed("rotate_right"):
		self.rotate_y(-1.0*delta)
