	; �T�E���h�T���v��

	; INES�w�b�_�[
	.inesprg 1 ;   - �v���O�����ɂ����̃o���N���g�����B���͂P�B
	.ineschr 1 ;   - �O���t�B�b�N�f�[�^�ɂ����̃o���N���g�����B���͂P�B
	.inesmir 0 ;   - �����~���[�����O
	.inesmap 0 ;   - �}�b�p�[�B�O�Ԃɂ���B

	; �[���y�[�W�ϐ�
Scroll_X = $00	; X�X�N���[���l
Sound_A = $01	; A�T�E���h�J�E���^
Sound_B = $02   ; B�T�E���h�J�E���^

	.bank 1      ; �o���N�P
	.org $FFFA   ; $FFFA����J�n

	.dw mainLoop ; VBlank���荞�݃n���h��(1/60�b����mainLoop���R�[�������)
	.dw Start    ; ���Z�b�g���荞�݁B�N�����ƃ��Z�b�g��Start�ɔ��
	.dw 0        ; �n�[�h�E�F�A���荞�݂ƃ\�t�g�E�F�A���荞�݂ɂ���Ĕ���

	.bank 0			 ; �o���N�O
	.org $0300	 ; $0300����J�n�A�X�v���C�gDMA�f�[�^�z�u
Sprite1_Y:     .db  0   ; �X�v���C�g#1 Y���W
Sprite1_T:     .db  0   ; �X�v���C�g#1 �i���o�[
Sprite1_S:     .db  0   ; �X�v���C�g#1 ����
Sprite1_X:     .db  0   ; �X�v���C�g#1 X���W
Sprite2_Y:     .db  0   ; �X�v���C�g#2 Y���W
Sprite2_T:     .db  0   ; �X�v���C�g#2 �i���o�[
Sprite2_S:     .db  0   ; �X�v���C�g#2 ����
Sprite2_X:     .db  0   ; �X�v���C�g#2 X���W


	.org $8000	 ; $8000����J�n
Start:
	; PPU�R���g���[�����W�X�^1������
	lda #%00001000	; �����ł�VBlank���荞�݋֎~
	sta $2000

waitVSync:
	lda $2002			; VBlank����������ƁA$2002��7�r�b�g�ڂ�1�ɂȂ�
	bpl waitVSync  ; bit7��0�̊Ԃ́AwaitVSync���x���̈ʒu�ɔ��Ń��[�v���đ҂�������


	; PPU�R���g���[�����W�X�^2������
	lda #%00000110	; ���������̓X�v���C�g��BG��\��OFF�ɂ���
	sta $2001

	; �p���b�g�����[�h
	ldx #$00    ; X���W�X�^�N���A

	; VRAM�A�h���X���W�X�^��$2006�ɁA�p���b�g�̃��[�h��̃A�h���X$3F00���w�肷��B
	lda #$3F
	sta $2006
	lda #$00
	sta $2006

loadPal:			; ���x���́A�u���x�����{:�v�̌`���ŋL�q
	lda tilepal, x ; A��(ourpal + x)�Ԓn�̃p���b�g�����[�h����

	sta $2007 ; $2007�Ƀp���b�g�̒l��ǂݍ���

	inx ; X���W�X�^�ɒl��1���Z���Ă���

	cpx #32 ; X��32(10�i���BBG�ƃX�v���C�g�̃p���b�g�̑���)�Ɣ�r���ē������ǂ�����r���Ă���	
	bne loadPal ;	�オ�������Ȃ��ꍇ�́Aloadpal���x���̈ʒu�ɃW�����v����
	; X��32�Ȃ�p���b�g���[�h�I��

	; ����(BG�̃p���b�g�w��f�[�^)�����[�h

	; $23C0�̑����e�[�u���Ƀ��[�h����
	lda #$23
	sta $2006
	lda #$C0
	sta $2006

	ldx #$00    ; X���W�X�^�N���A
	lda #%00000000				; �S�Ƃ��p���b�g0��
	; 0�Ԃ�1�Ԃɂ���
loadAttrib
	eor #%01010101				; XOR���Z�ň�����̃r�b�g�����݂ɂO���P�ɂ���
	sta $2007							; $2007�ɑ����̒l($0��$55)��ǂݍ���
	; 64��(�S�L�����N�^�[��)���[�v����
	inx
	cpx #64
	bne loadAttrib

	; �l�[���e�[�u������

	; $2000�̃l�[���e�[�u���ɐ�������
	lda #$20
	sta $2006
	lda #$00
	sta $2006

	lda #$00        ; 0��(�^����)
	ldy #$00    ; Y���W�X�^������
loadNametable1:
	ldx Star_Tbl, y			; Star�e�[�u���̒l��X�ɓǂݍ���
loadNametable2:
	sta $2007				; $2007�ɑ����̒l��ǂݍ���
	dex							; X���Z
	bne loadNametable2	; �܂�0�łȂ��Ȃ�΃��[�v���č����o�͂���
	; 1�Ԃ�2�Ԃ̃L������Y�̒l������݂Ɏ擾
	tya							; Y��A
	and #1					; A AND 1
	adc #1					; A��1���Z����1��2��
	sta $2007				; $2007�ɑ����̒l��ǂݍ���
	lda #$00        ; 0��(�^����)
	iny							; Y���Z
	cpy #20					; 20��(���e�[�u���̐�)���[�v����
	bne loadNametable1

	; �P�Ԗڂ̃X�v���C�g���W������
	lda X_Pos_Init
	sta Sprite1_X
	lda Y_Pos_Init
	sta Sprite1_Y
	; �Q�Ԗڂ̃X�v���C�g���W�X�V�T�u���[�`�����R�[��
	jsr setSprite2
	; �Q�Ԗڂ̃X�v���C�g�𐅕����]
	lda #%01000000
	sta Sprite2_S

	; PPU�R���g���[�����W�X�^2������
	lda #%00011110	; �X�v���C�g��BG�̕\����ON�ɂ���
	sta $2001

	; �T�E���h���W�X�^������
	lda #0
	sta $4015

	; PPU�R���g���[�����W�X�^1�̊��荞�݋��t���O�𗧂Ă�
	lda #%10001000
	sta $2000

infinityLoop:					; VBlank���荞�ݔ�����҂����̖������[�v
	jmp infinityLoop

mainLoop:					; ���C�����[�v

	; �X�v���C�g�`��(DMA�𗘗p)
	lda #$3  ; �X�v���C�g�f�[�^��$0300�Ԓn����Ȃ̂ŁA3�����[�h����B
	sta $4014 ; �X�v���C�gDMA���W�X�^��A���X�g�A���āA�X�v���C�g�f�[�^��DMA�]������
	
	; BG�X�N���[��
	lda $2002			; �X�N���[���l�N���A
	lda <Scroll_X	; X�̃X�N���[���l�����[�h
	sta $2005			; X�����X�N���[���iY�����͌Œ�)
	inc <Scroll_X	; �X�N���[���l�����Z

	; �p�b�hI/O���W�X�^�̏���
	lda #$01
	sta $4016
	lda #$00
	sta $4016

	; �p�b�h���̓`�F�b�N
	lda $4016  ; A�{�^��
	and #1     ; AND #1
	beq isBKEYdown  ; 0�Ȃ��IsBKEYdown�փW�����v
	jsr AKEYSound

isBKEYdown:
	lda $4016  ; B�{�^��
	and #1     ; AND #1
	beq isSelectdown  ; 0�Ȃ��isSelectdown�փW�����v
	jsr BKEYSound

isSelectdown
	lda $4016  ; Select�{�^�����X�L�b�v
	lda $4016  ; Start�{�^�����X�L�b�v
	lda $4016  ; ��{�^��
	and #1     ; AND #1
	bne UPKEYdown  ; 0�łȂ��Ȃ�Ή�����Ă�̂�UPKeydown�փW�����v
	
	lda $4016  ; ���{�^��
	and #1     ; AND #1
	bne DOWNKEYdown ; 0�łȂ��Ȃ�Ή�����Ă�̂�DOWNKeydown�փW�����v

	lda $4016  ; ���{�^��
	and #1     ; AND #1
	bne LEFTKEYdown ; 0�łȂ��Ȃ�Ή�����Ă�̂�LEFTKeydown�փW�����v

	lda $4016  ; �E�{�^��
	and #1     ; AND #1
	bne RIGHTKEYdown ; 0�łȂ��Ȃ�Ή�����Ă�̂�RIGHTKeydown�փW�����v
	jmp NOTHINGdown  ; �Ȃɂ�������Ă��Ȃ��Ȃ��NOTHINGdown��

UPKEYdown:
	dec Sprite1_Y	; Y���W��1���Z
	jmp NOTHINGdown

DOWNKEYdown:
	inc Sprite1_Y ; Y���W��1���Z
	jmp NOTHINGdown

LEFTKEYdown:
	dec Sprite1_X	; X���W��1���Z
	jmp NOTHINGdown 

RIGHTKEYdown:
	inc Sprite1_X	; X���W��1���Z
	; ���̌�NOTHINGdown�Ȃ̂ŃW�����v����K�v����

NOTHINGdown:
	; �Q�Ԗڂ̃X�v���C�g���W�X�V�T�u���[�`�����R�[��
	jsr setSprite2

	; �T�E���h�҂��J�E���^
	lda <Sound_A
	beq dec_B
	dec <Sound_A
dec_B:
	lda <Sound_B
	beq NMIEnd
	dec <Sound_B

NMIEnd:
	rti									; ���荞�݂��畜�A

AKEYSound:
	; �T�E���h�҂��J�E���^A��0�łȂ��ꍇ�̓T�E���h��炳�Ȃ�
	lda <Sound_A
	beq AKEYSoundSub
	rts
AKEYSoundSub:
	lda #10			; 1/6�b��1��炷
	sta <Sound_A

	lda $4015		; �T�E���h���W�X�^
	ora #%00000001	; ��`�g�`�����l���P��L���ɂ���
	sta $4015

	lda #%10111111
	sta $4000		; ��`�g�`�����l���P���䃌�W�X�^�P

	lda #%10101011
	sta $4001		; ��`�g�`�����l���P���䃌�W�X�^�Q
	lda Sprite1_X		; ���V�т�X���W�����Ă݂�
	sta $4002		; ��`�g�`�����l���P���g�����W�X�^�P

	lda #%11111011
	sta $4003		; ��`�g�`�����l���P���g�����W�X�^�Q

	rts

BKEYSound:
	; �T�E���h�҂��J�E���^B��0�łȂ��ꍇ�̓T�E���h��炳�Ȃ�
	lda <Sound_B
	beq BKEYSoundSub
	rts
BKEYSoundSub:
	lda #10			; 1/6�b��1��炷
	sta <Sound_B

	lda $4015		; �T�E���h���W�X�^
	ora #%00000010	; ��`�g�`�����l���Q��L���ɂ���
	sta $4015

	lda #%10111111
	sta $4004		; ��`�g�`�����l���Q���䃌�W�X�^�P

	lda #%10000100
	sta $4005		; ��`�g�`�����l���Q���䃌�W�X�^�Q
	lda Sprite1_Y		; ���V�т�Y���W�����Ă݂�
	sta $4006		; ��`�g�`�����l���Q���g�����W�X�^�P

	lda #%11111000
	sta $4007		; ��`�g�`�����l���Q���g�����W�X�^�Q

	rts

setSprite2:
	; �Q�Ԗڂ̃X�v���C�g�̍��W�X�V�T�u���[�`��
	clc					;�@adc�̑O�ɃL�����[�t���O���N���A
	lda Sprite1_X
	adc #8 		; 8�ޯĉE�ɂ��炷
	sta Sprite2_X
	lda Sprite1_Y
	sta Sprite2_Y
	rts

	; �����f�[�^
X_Pos_Init   .db 20       ; X���W�����l
Y_Pos_Init   .db 40       ; Y���W�����l

	; ���e�[�u���f�[�^(20��)
Star_Tbl    .db 60,45,35,60,90,65,45,20,90,10,30,40,65,25,65,35,50,35,40,35

tilepal: .incbin "giko2.pal" ; �p���b�g��include����

	.bank 2       ; �o���N�Q
	.org $0000    ; $0000����J�n

	.incbin "giko2.bkg"  ; �w�i�f�[�^�̃o�C�i���B�t�@�C����include����
	.incbin "giko2.spr"  ; �X�v���C�g�f�[�^�̃o�C�i���B�t�@�C����include����
